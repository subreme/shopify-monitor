// This is where the logic for the actual monitor will be.

use crate::{
    default, error, hidden,
    message::*,
    products::{File, Product},
    stores::Store,
    success, warning,
    webhook::{self, Status},
};
use base64::{encode_config, URL_SAFE};
use chrono::prelude::*;
use futures::future::join_all;
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::{
        mpsc::{self, Sender},
        oneshot, RwLock,
    },
    task,
    time::{self, sleep},
};

pub async fn run(stores: Vec<Store>) {
    // This variable will keep track of the number of stores being
    // monitored so that the program can quit if it drops to zero.
    let amount = stores.len();

    default!("Monitoring {} stores...", amount);

    // While after some testing I realized that collecting tasks in a
    // vector and `join`ing them is not necessary for them to run, the
    // program will quit immediately as it will not "wait for them to
    // complete", therefore I'm keeping this. While other methods exist
    // to avoid this, maintaining the `tasks` vector should make it
    // simpler to have the program quit once new features are introduced.
    let mut tasks = vec![];

    // This vector will list all invalid webhook links so that the
    // program does not waste time sending requests to them. It uses a
    // `RwLock` so that all tasks can read its contents while only one
    // of them can edit it (adding more links) at once. In order to
    // maximize performance, the current strategy is to check that the
    // vector's length has not changed before each iteration of the
    // monitoring task, and to check if the newly black-listed webhooks
    // are part of the list the task is supposed to send to. If they
    // are, they will be removed from said list, and the monitor will
    // not attempt t send updates to it again. If any webhook-sending
    // task receives an invalid status, the link will be added to the
    // `broken_webhooks` list so that all tasks can remove it
    // immediately.
    let broken_webhooks = Arc::new(RwLock::new(vec![]));

    // This channel will be used to allow monitoring tasks to
    // communicate with a background task. While more functionality will
    // be added to it in the future (such as detecting any updates to
    // This counter will be used the config file), it currently only
    // serves as a counter to keep track of how many stores the monitor
    // is unable to connect to at any given time, so that if its value
    // reaches the total number of stores, the program will know is
    // offline. The value of its buffer is arbitrary and may be subject
    // to change following further testing.
    let (tx, mut rx) = mpsc::channel(amount * 5);

    // This channel, on the other hand, is meant to be used only once in
    // order to force the `run()` function to return, terminating the
    // monitor.
    let (quit_tx, quit_rx) = oneshot::channel();

    // `amount` is re-declared here as it needs to be an `Arc<>` but it's
    // inconvenient to have to `.read()` it in the statements between
    // when it is first declared and here.
    let amount = Arc::new(RwLock::new(amount));

    // This code block is sectioned-off so that `amount` can be cloned
    // without renaming it.
    {
        let amount = amount.clone();
        let broken_webhooks = broken_webhooks.clone();

        // This "background task" receives updates from the other tasks
        // through a `mpsc` channel and handles them so that the monitor
        // can run without interruptions.
        tasks.push(task::spawn(async move {
            let mut offline = 0;

            while let Some(update) = rx.recv().await {
                match update {
                    // Monitor Updates
                    Update::Monitor(MonitorUpdate::Quit, message) => {
                        error!("{}", message);
                        break;
                    }

                    // Site Updates
                    Update::Site(SiteUpdate::Online, _) => {
                        offline -= 1;

                        if offline == 0 {
                            success!("All sites are are back online!");
                        }
                    }

                    Update::Site(SiteUpdate::Offline, _) => {
                        offline += 1;

                        if offline == *amount.read().await {
                            error!("The program seems not to be connected to the Internet!");
                        }
                    }

                    // Webhook Updates
                    Update::Webhook(_, url) => {
                        warning!("Invalid webhook: {}!", url);

                        // If the webhook is invalid, it's added to this vector so
                        // that the program will stop sending requests to it.
                        broken_webhooks.write().await.push(url.clone());
                    }

                    #[allow(unreachable_patterns)]
                    _ => {}
                }
            }

            // If the loop is ended, a message will be sent through
            // `quit_tx` and the monitor will stop.
            quit_tx.send(()).expect("Failed to send message.");
        }));
    }

    for store in stores {
        let broken_webhooks = broken_webhooks.clone();
        let tx = tx.clone();
        let amount = amount.clone();

        // These vectors contain all channels the monitor should send
        // webhooks to, divided by the type of events included.
        let restock = Arc::clone(&store.restock);
        let password_up = Arc::clone(&store.password_up);
        let password_down = Arc::clone(&store.password_down);

        tasks.push(task::spawn(async move {
            let client = reqwest::Client::new();

            // By tracking the value of these variables, the monitor can
            // detect any changes and send out webhooks accordingly.
            let mut previous: Option<Vec<MinimalProduct>> = None;
            let mut password_page = false;
            let mut rate_limit = false;
            let mut online = true;
            let mut broken_prev = 0;

            // This will be used to return `Future`s that complete at
            // intervals as long as the `delay` specified by the user.
            let mut interval = time::interval(Duration::from_millis(store.delay));

            // This `loop` is named so that it can be `break`ed out of
            // from within another loop.
            'main: loop {
                // In this version of the monitor, when a webhook is
                // detected to be invalid it is removed from the list of
                // links that the program sends requests to. Checking
                // the list takes some time, as the list has to be
                // checked at every iteration, but skipping the process
                // of sending unnecessary requests should make up for
                // it. If testing shows that there is performance is
                // negatively affected, this feature will be removed.

                // The webhooks are only checked if new links have been
                // blacklisted.
                let broken_curr = broken_webhooks.read().await;

                if broken_curr.len() != broken_prev {
                    // Only the newly "banned" webhooks are checked.
                    for i in broken_prev..broken_curr.len() {
                        // Since links are never removed from the
                        // vector, it isn't possible for the length of
                        // `broken_webhooks` to decrease, so it's safe
                        // to access the elements using square brackets.

                        // The URL is checked instead of the channel
                        // itself because if two user-created "channels"
                        // were to share the same link, both should be
                        // removed.
                        restock
                            .write()
                            .await
                            .retain(|c| c.url != broken_curr[i]);
                        password_up
                            .write()
                            .await
                            .retain(|c| c.url != broken_curr[i]);
                        password_down
                            .write()
                            .await
                            .retain(|c| c.url != broken_curr[i]);

                        // If nothing is being monitored (as there
                        // aren't any valid webhooks to send updates to)
                        // the task is killed.
                        if restock.read().await.is_empty() &&
                            password_up.read().await.is_empty() &&
                            password_down.read().await.is_empty() {
                            break 'main;
                        }
                    }

                    // The variable keeping track of the amount of
                    // banned links should be updated or the program
                    // will always check every vector for no reason.
                    broken_prev = broken_curr.len();
                }

                // The endpoint for all Shopify store is
                // `/products.json`, so it has to be added to the
                // website's URL to get the link to it.
                let req = client.get(
                    /* format!("{}/products.json?limit=100", */
                    format!("{}/products.json",
                    &store.url.clone()
                ))

                    // For this first version, I simply "borrowed" the "Safe
                    // Headers" used in his JavaScript Shopify Monitor, however I
                    // will experiment with more techniques to avoid bot detection
                    // later. Here's the link to his repository:
                    // https://github.com/aarock1234/shopify-monitor/blob/master/src/class/monitor.js.
                    .header("pragma", "no-cache")
                    .header("cache-control", "no-cache")
                    .header("upgrade-insecure-requests", "1")
                    .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.198 Safari/537.36") 
                    .header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9") 
                    .header("sec-fetch-site", "none")
                    .header("sec-fetch-mode", "navigate")
                    .header("sec-fetch-user", "?1")
                    .header("sec-fetch-dest", "document")
                    .header("accept-language", "en-US,en;q=0.9")

                    .send()
                    .await;

                if let Ok(res) = req {
                    /* hidden!("Fetched {}! Status: {}!", res.url(), res.status()); */

                    if !online {
                        default!("`{}` is back online!", store.name);
                        tx.send(Update::Site(SiteUpdate::Online, "".into())).await.expect("Failed to send update.");
                        online = true;
                    }

                    if res.status() == 200 {
                        // In this case, a webhook saying the password
                        // page is down will be sent.
                        if password_page {
                            password_page = false;

                            hidden!("Password page raised on {}!", store.url);
                            success!("{}: Password Page Up!", store.name);

                            // This variable keeps track of the number
                            // of webhooks sent for each store update.
                            let mut quantity = 0;

                            // The program will cycle through each
                            // channel that should be notified and send
                            // out a webhook.
                            for channel in password_down.read().await.iter() {
                                task::spawn(password(PasswordSettings {
                                    kind: Password::Down,
                                    url: channel.url.clone(),
                                    username: channel.settings.username.clone(),
                                    avatar: channel.settings.avatar.clone(),
                                    color: channel.settings.color,
                                    footer_text: channel.settings.footer_text.clone(),
                                    footer_image: channel.settings.footer_image.clone(),
                                    timestamp: channel.settings.timestamp,
                                    store_name: store.name.clone(),
                                    store_url: store.url.clone(),
                                    store_logo: store.logo.clone(),
                                    broken_webhooks: broken_webhooks.clone(),
                                    tx: tx.clone(),
                                }));

                                // I think that using a counter should
                                // be faster than accessing the
                                // `password_down` vector to check its
                                // length, but I may be wrong.
                                quantity += 1;
                            }

                            default!(
                                "Sending {} webhook{}...",
                                quantity,
                                // This conditional statement appends an
                                // "s" to the word "webhook" if more
                                // than one is sent. I'm using `\0`, a
                                // null character, instead of an empty
                                // character as the latter doesn't
                                // exist.
                                // https://stackoverflow.com/questions/3670505/why-is-there-no-char-empty-like-string-empty
                                if quantity == 1 { '\0' } else { 's' }
                            );
                        }

                        if rate_limit {
                            rate_limit = false;
                        }

                        let json = res.json::<File>().await;

                        if let Ok(current) = json {
                            // The current products have to allow for
                            // multiple owners as they are consumed by
                            // iterators when checking their contents,
                            // but need to be used again later to update
                            // the previous ones.
                            let current_products = Arc::new(current.products);

                            // If there's a previous value for the
                            // response to be compared to, the stock
                            // changes can be extracted and restock
                            // webhooks can be sent out.
                            if let Some(previous) = previous {
                                // Cycling through each current
                                // product the program finds the
                                // same item in the previous ones by
                                // matching IDs.
                                for curr in (*current_products).iter() {
                                    // Originally, when the program
                                    // found a product that had been
                                    // updated, it would also check if
                                    // the availability of any variant
                                    // has changed (becoming available).
                                    // I changed how it functions,
                                    // though, as it's usually more
                                    // useful to have links to every
                                    // available variant (often the
                                    // "size" of a product), rather than
                                    // only the ones that were
                                    // previously unavailable.

                                    // By removing this logic, however,
                                    // a problem emerged: a "duplicate"
                                    // webhook would be sent whenever a
                                    // single variant went out of stock,
                                    // as that counts as a product
                                    // update.

                                    // To counter this, the program now
                                    // also checks that at least one
                                    // unavailable variant has become
                                    // available (or rather, that one of
                                    // the available variants used to
                                    // be unavailable), before sending a
                                    // webhook.
                                    if let Some(prev) =
                                        previous.iter().find(|prev| prev.id == curr.id)
                                    {
                                        if curr.updated_at != prev.updated_at
                                            && curr.variants.iter().any(|curr| {
                                                prev.variants.iter().any(|prev| {
                                                    prev.id == curr.id
                                                        && !prev.available
                                                        && curr.available
                                                })
                                            })
                                        {
                                            /* hidden!("Product {} Updated At: {}", curr.id, curr.updated_at); */

                                            hidden!("{}/product/{} restocked!", store.url, curr.id);
                                            success!("{}: `{}` restocked!", store.name, curr.title);

                                            let mut quantity = 0;

                                            let ap = available_product(curr);

                                            for channel in restock.read().await.iter() {
                                                if curr
                                                    .variants
                                                    .iter()
                                                    .filter(|v| v.available)
                                                    .count()
                                                    >= channel.settings.minimum
                                                {
                                                    // Although it may
                                                    // not seem like it
                                                    // at first glance,
                                                    // `item()` is a
                                                    // function, and
                                                    // `webhooks`
                                                    // contains an
                                                    // asynchronous
                                                    // function for each
                                                    // webhook that
                                                    // should be sent.
                                                    task::spawn(item(ItemSettings {
                                                        kind: Item::Restock,
                                                        product: ap.clone(),
                                                        url: channel.url.clone(),
                                                        username: channel.settings.username.clone(),
                                                        avatar: channel.settings.avatar.clone(),
                                                        color: channel.settings.color,
                                                        sizes: channel.settings.sizes,
                                                        thumbnail: channel.settings.thumbnail,
                                                        image: channel.settings.image,
                                                        footer_text: channel
                                                            .settings
                                                            .footer_text
                                                            .clone(),
                                                        footer_image: channel
                                                            .settings
                                                            .footer_image
                                                            .clone(),
                                                        timestamp: channel.settings.timestamp,
                                                        store_name: store.name.clone(),
                                                        store_url: store.url.clone(),
                                                        store_logo: store.logo.clone(),
                                                        broken_webhooks: broken_webhooks.clone(),
                                                        tx: tx.clone(),
                                                    }));

                                                    /* hidden!("Pushed a webhook for product {}!", curr.id); */

                                                    quantity += 1;
                                                }
                                            }

                                            /* hidden!("Sending webhooks for `{}`!", curr.id); */

                                            default!(
                                                "Sending {} webhook{}...",
                                                quantity,
                                                if quantity == 1 { '\0' } else { 's' }
                                            );
                                        }

                                    // This code will run if a
                                    // product is found that wasn't
                                    // present among the previous
                                    // ones, meaning it's a new one.
                                    } else {
                                        hidden!("{}/product/{} was added!", store.url, curr.id);
                                        success!("{}: `{}` was added!", store.name, curr.title);

                                        let mut quantity = 0;

                                        let ap = available_product(curr);

                                        for channel in restock.read().await.iter() {
                                            task::spawn(item(ItemSettings {
                                                kind: Item::New,
                                                product: ap.clone(),
                                                url: channel.url.clone(),
                                                username: channel.settings.username.clone(),
                                                avatar: channel.settings.avatar.clone(),
                                                color: channel.settings.color,
                                                sizes: channel.settings.sizes,
                                                thumbnail: channel.settings.thumbnail,
                                                image: channel.settings.image,
                                                footer_text: channel.settings.footer_text.clone(),
                                                footer_image: channel.settings.footer_image.clone(),
                                                timestamp: channel.settings.timestamp,
                                                store_name: store.name.clone(),
                                                store_url: store.url.clone(),
                                                store_logo: store.logo.clone(),
                                                broken_webhooks: broken_webhooks.clone(),
                                                tx: tx.clone(),
                                            }));

                                            quantity += 1;
                                        }

                                        default!(
                                            "Sending {} webhook{}...",
                                            quantity,
                                            if quantity == 1 { '\0' } else { 's' }
                                        );
                                    }
                                }
                            }

                            // On the monitor's first run, there
                            // won't be a `previous` value therefore
                            // it will have to be saved for the
                            // first time. This doesn't occur in an
                            // `else` block, though, as the value
                            // has to be updated on every cycle
                            // regardless.
                            previous = minimal_products(current_products);
                        } else if let Err(e) = json {
                            hidden!("Failed to parse JSON for {}: {}", store.url, e);

                            // The program will wait for the interval to complete
                            // its cycle before running the next iteration and
                            // fetching the store's products again.
                            interval.tick().await;
                            continue;
                        };

                        // In this case, a webhook with the restocked
                        // items will be sent.
                    } else if res.status() == 401 {
                        // In this case, a webhook saying the password
                        // page is up will be sent.
                        if !password_page {
                            password_page = true;

                            hidden!("Password page raised on {}!", store.url);
                            success!("{}: Password Page Up!", store.name);

                            let mut quantity = 0;

                            // The program will cycle through each
                            // channel that should be notified and send
                            // out a webhook.
                            for channel in password_up.read().await.iter() {
                                task::spawn(password(PasswordSettings {
                                    kind: Password::Up,
                                    url: channel.url.clone(),
                                    username: channel.settings.username.clone(),
                                    avatar: channel.settings.avatar.clone(),
                                    color: channel.settings.color,
                                    footer_text: channel.settings.footer_text.clone(),
                                    footer_image: channel.settings.footer_image.clone(),
                                    timestamp: channel.settings.timestamp,
                                    store_name: store.name.clone(),
                                    store_url: store.url.clone(),
                                    store_logo: store.logo.clone(),
                                    broken_webhooks: broken_webhooks.clone(),
                                    tx: tx.clone(),
                                }));

                                quantity += 1;
                            }

                            default!(
                                "Sending {} webhook{}...",
                                quantity,
                                if quantity == 1 { '\0' } else { 's' }
                            );
                        }
                    } else if res.status() == 429 && !rate_limit {
                        rate_limit = true;
                        warning!("Rate limit reached for {}!", store.name);
                    }
                } else if online {
                    warning!("Failed to GET {}!", store.url);
                    tx.send(Update::Site(SiteUpdate::Offline, "".into())).await.expect("Failed to send update.");
                    online = false;
                }

                // The program will wait for the interval to complete
                // its cycle before running the next iteration and
                // fetching the store's products again.
                interval.tick().await;
            };

            error!("All webhook URLs for `{}` are invalid!", store.name);
            default!("Stopped monitoring {}.", store.url);
            *amount.write().await -= 1;

            // If no stores are being monitored, the `run()` function
            // will return and the program will quit.
            if *amount.read().await == 0 {
                tx.send(Update::Monitor(MonitorUpdate::Quit, "No valid webhooks!".into())).await.expect("Failed to send update.");
            }
        }));
    }

    if quit_rx.await.is_ok() {
        return;
    }

    // This function call ensures that the program doesn't exit while
    // the monitor is still running.
    join_all(tasks).await;
}

#[derive(Debug)]
enum Update {
    Monitor(MonitorUpdate, String),
    Site(SiteUpdate, String),
    Webhook(WebhookUpdate, String),
}

#[derive(Debug)]
enum MonitorUpdate {
    Quit,
}

#[derive(Debug, PartialEq)]
enum SiteUpdate {
    Online,
    Offline,
}

#[derive(Debug)]
enum WebhookUpdate {
    Invalid,
}

fn minimal_products(current_products: Arc<Vec<Product>>) -> Option<Vec<MinimalProduct>> {
    Some({
        let mut products = vec![];
        for product in (*current_products).iter() {
            let mut variants = vec![];
            for variant in &product.variants {
                variants.push(MinimalVariant {
                    id: variant.id,
                    available: variant.available,
                    /* updated_at: variant.updated_at.clone(), */
                });
            }
            products.push(MinimalProduct {
                id: product.id,
                updated_at: product.updated_at.clone(),
                variants,
            });
        }
        products
    })
}

// When previous products and variants are compared against the current ones to find
// any changes, only their id (used to identify them) and the time they
// were last updated (do determine if it's even possible for their stock
// number to have changed) are compared. This new struct, holding the
// minimum amount of data, can be used to reduce memory usage so that
// products don't have to be saved to a database.
struct MinimalProduct {
    id: u64,
    updated_at: String,
    variants: Vec<MinimalVariant>,
}

// The fields of this struct used to be public while those of `MinimalProduct`
// are not because a test required it.

struct MinimalVariant {
    id: u64,
    available: bool,
    // While the program could check when each variant was last updated,
    // ignoring that value and only checking its availability is faster,
    // and removing its field results in lower memory usage.
    // updated_at: String,
}

// Since the monitor will check which variants are available for a
// certain product before sending a webhook, a different struct is
// needed to send as `item()`'s parameter, or the program will have to
// perform those checks again. `AvailableProduct` will therefore only
// contain the product's information that should be included in the
// webhook's embed, as well as a vector containing the available
// variants.
#[derive(PartialEq, Debug)]
struct AvailableProduct {
    name: String,

    // The product's handle can be used to obtain the product link as
    // follows: `format!("{}/products/{}", store_url, handle)`.
    handle: String,
    brand: String,
    price: String,

    // I changed this to an `Option` as for some reason (which I can't
    // remember) I was using an empty `String` instead of `None` if the
    // product didn't have a photo.
    image: Option<String>,
    variants: Vec<AvailableVariant>,
}

// There's no need to make unnecessary operations or clone unused data,
// so this struct holds the bare minimum. Since some values
#[derive(PartialEq, Debug)]
struct AvailableVariant {
    name: String,
    id: u64,
}

// Why do two `struct`s for both "Minimal" and "Available" Products and
// Variants exist, if they share the goal to reduce memory usage and are
// so similar? Their similarity is due to their common goal, but they
// aim for it in different contexts. The "Minimal" `struct`s are the
// smallest, and contain the data required for the program to check for
// product updates. The "Available" `struct`s, on the other hand, hold
// different data types, as they include the product details used to
// form webhook embeds. As a result, both types are needed.

fn available_product(
    curr: &Product, /*, prev: Option<&Vec<MinimalVariant>>*/
) -> Arc<AvailableProduct> {
    let mut variants: Vec<AvailableVariant> = vec![];

    let price = if let Some(v) = curr.variants.get(0) {
        v.price.clone()
    } else {
        // The default price value is "?" because it must be at least 1
        // character long.
        "?".into()
    };

    /*
    let image = if let Some(img) = curr.images.get(0) {
        Some(img.src.clone())
    } else {
        None
    };
    */

    let image = curr.images.get(0).map(|img| img.src.clone());

    for variant in curr.variants.iter() {
        if variant.available {
            variants.push(AvailableVariant {
                // Some websites have very weird variant names.
                // UNDEFEATED, for example, prefixes their "sizes" with
                // "- / ". `.trim_prefix()` cannot be used to correct
                // this, as it would only work for specific cases. By
                // removing all special characters, so that the name
                // only contains letters, numbers, and whitespace,
                // almost all of these strange names can be "normalized".
                name: variant
                    .title
                    // The string is split into characters.
                    .chars()
                    // The split is transformed into a vector.
                    .collect::<Vec<char>>()
                    // The program can now iterate through each char.
                    .iter()
                    // "Invalid" characters are removed.
                    .filter(|c| c.is_alphanumeric() || c.is_whitespace() || c == &&'.')
                    // The filtered characters are collected into a string.
                    .collect::<String>()
                    // Leading and trailing whitespace is removed.
                    .trim()
                    // The returned `&str` is converted to a `String`.
                    .into(),
                id: variant.id,
            });
        }
    }

    Arc::from(AvailableProduct {
        name: curr.title.clone(),
        handle: curr.handle.clone(),
        brand: curr.vendor.clone(),
        price,
        image,
        variants,
    })
}

// This function is called by `product()` and `password()` functions,
// incorporating the logic used in both of them to send webhooks. Its
// only parameters are the webhook's URL and the `Message` to be sent,
// while the two functions' role is to construct the embeds, as they
// will differ between item and password-related notifications.
async fn request(
    url: String,
    msg: Arc<Message>,
    broken: Arc<RwLock<Vec<String>>>,
    tx: Sender<Update>,
) {
    /* hidden!("Webhook Preview: {}", preview(msg.clone())); */

    loop {
        match webhook::send(url.clone(), msg.clone()).await {
            // Seems like the compiler complains when I use my logging
            // macros outside of code blocks...
            /* Status::Success => hidden!("Successfully sent webhook to {}!", url), */
            Status::Success => {
                hidden!("Successfully sent webhook to {}!", url);
            }
            Status::RateLimit(seconds) => {
                hidden!("Rate Limit reached for {}!", url);

                if let Some(seconds) = seconds {
                    hidden!("Waiting {} seconds for {}...", seconds, url);
                    sleep(Duration::from_secs_f64(seconds)).await;

                    // The loop only iterates again if the webhook is
                    // rate-limited and the program can determine how
                    // long to wait for. In all other cases, the
                    // function returns and the task is terminated.
                    continue;
                }
            }
            Status::Invalid => {
                if !broken.read().await.contains(&url) {
                    // Due to the channel's buffer, sending this message
                    // should take less time than `.write()`ing to
                    // `broken` directly.
                    tx.send(Update::Webhook(WebhookUpdate::Invalid, url))
                        .await
                        .expect("Failed to send update.");
                }
            }

            // I'm not sure what the program should do when an unknown
            // error occurs. Since in some cases it might be best to
            // treat it the same way as an invalid webhook (as the issue
            // should be persistent), I might chang this `match`
            // statement's logic or introduce a new setting to decide
            // the monitor's behavior.
            Status::Unknown => {
                warning!("Failed to send webhook to {}!", url);
            }
        }

        return;
    }
}

// The function and enum are named `item()` and `Item`, and not
// `product()` and `Product`, because the `Product` name is already used
// by `crate::products::Product`, which is named after `products.json`.
async fn item(settings: ItemSettings) {
    /* hidden!("`item()` started for {}!", product.name.clone()); */

    let embed = Embed {
        title: Some(settings.product.name.clone()),
        description: None,
        url: Some(format!(
            "{}/products/{}",
            settings.store_url, settings.product.handle
        )),
        color: settings.color,
        fields: {
            let quantity = if settings.sizes {
                /*
                let len = 3 + product.variants.len();

                if len % 3 == 2 {
                  len + 4
                }

                len + 3
                */

                // Since the checks for the number of variants (above
                // this comment) were removed, the number of fields
                // is always 4 more than the number of variants, and
                // is the vector holding them occasionally has a
                // slightly larger capacity than necessary.
                settings.product.variants.len() + 4
            } else {
                3
            };

            // The value calculated above is used to preallocate the
            // correct amount of space in the heap to hold this
            // vector, slightly improving performance.
            let mut fields = Vec::with_capacity(quantity);

            fields.push(Field {
                name: "Event".into(),
                inline: Some(true),
                value: {
                    if settings.kind == Item::New {
                        "New Product".into()
                    } else {
                        "Restock".into()
                    }
                },
            });

            fields.push(Field {
                name: "Brand".into(),
                inline: Some(true),
                value: settings.product.brand.clone(),
            });

            fields.push(Field {
                name: "Price".into(),
                inline: Some(true),
                value: settings.product.price.clone(),
            });

            /* hidden!("{} has {} updated variants!", settings.product.name, settings.product.variants.len()); */

            if settings.sizes {
                for variant in (*settings.product.variants).iter() {
                    fields.push(Field {
                        name: format!("Size {}", variant.name),
                        inline: Some(true),
                        value: format!("[ATC]({}/cart/add?id={})", settings.store_url, variant.id),
                    });
                }

                // When the bottom row of a Discord embed
                // has two fields, it is aligned differently from
                // the other rows, which some users consider
                // displeasing. As a test, it is currently always
                // "corrected" by the program, which adds an
                // invisible field when necessary. In a future
                // update, a toggle may be added allowing users to
                // opt out of this behavior.
                if fields.len() % 3 == 2 {
                    fields.push(Field {
                        // The characters held by the `name` and
                        // `value` fields are the `U+2800` "Braille
                        // Pattern Blank" character, which can be
                        // used to fool Discord into thinking that
                        // they aren't blank.
                        name: '⠀'.into(),
                        inline: Some(true),
                        value: '⠀'.into(),
                    });
                }
            }

            Some(fields)
        },
        author: Some(Author {
            name: settings.store_name,
            url: Some(settings.store_url.clone()),
            icon_url: Some(settings.store_logo),
        }),
        footer: {
            // The program doesn't check if a footer image was
            // included, as if a timestamp or footer text
            // weren't, it won't be rendered regardless.
            if settings.footer_text.is_some() || settings.timestamp {
                Some(Footer {
                    text: settings.footer_text,
                    icon_url: settings.footer_image,
                })
            } else {
                None
            }
        },
        timestamp: {
            if settings.timestamp {
                Some(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true))
            } else {
                None
            }
        },
        image: {
            // This isn't very elegant, but I copied it from the
            // `thumbnail` field below where it was the only
            // solution I found.

            let mut img = None;
            if settings.image && settings.product.image.is_some() {
                img = Some(Image {
                    url: settings
                        .product
                        .image
                        .clone()
                        .expect("Failed to extract Image URL."),
                });
            }
            img
        },
        thumbnail: {
            let mut tn = None;
            if settings.thumbnail && settings.product.image.is_some() {
                tn = Some(Thumbnail {
                    url: settings
                        .product
                        .image
                        .clone()
                        .expect("Failed to extract Image URL."),
                });
            }
            tn
        },
    };

    let msg = Arc::from(Message {
        content: None,
        embeds: Some(vec![embed]),
        username: settings.username,
        avatar_url: settings.avatar.clone(),
    });

    /* hidden!("Calling `request()` for {}!", product.name.clone()); */

    request(settings.url, msg, settings.broken_webhooks, settings.tx).await;
}

struct ItemSettings {
    kind: Item,
    product: Arc<AvailableProduct>,
    url: String,
    username: Option<String>,
    avatar: Option<String>,
    color: Option<u32>,
    sizes: bool,
    /* atc: Option<bool>, */
    thumbnail: bool,
    image: bool,
    footer_text: Option<String>,
    footer_image: Option<String>,
    timestamp: bool,
    store_name: String,
    store_url: String,
    store_logo: String,
    broken_webhooks: Arc<RwLock<Vec<String>>>,
    tx: Sender<Update>,
}

#[derive(PartialEq)]
enum Item {
    New,
    Restock,
}

async fn password(settings: PasswordSettings) {
    let embed = Embed {
        title: Some(format!("Password Page {}!", {
            if settings.kind == Password::Up {
                "Up"
            } else {
                "Down"
            }
        })),
        description: None,
        url: Some(settings.store_url.clone()),
        color: settings.color,
        fields: None,
        author: Some(Author {
            name: settings.store_name,
            url: Some(settings.store_url.clone()),
            icon_url: Some(settings.store_logo),
        }),
        footer: {
            // The program doesn't check if a footer image was included,
            // as if a timestamp or footer text weren't, it won't be
            // rendered regardless.
            if settings.footer_text.is_some() || settings.timestamp {
                Some(Footer {
                    text: settings.footer_text,
                    icon_url: settings.footer_image,
                })
            } else {
                None
            }
        },
        timestamp: {
            if settings.timestamp {
                Some(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true))
            } else {
                None
            }
        },
        image: None,
        thumbnail: None,
    };

    let msg = Arc::from(Message {
        content: None,
        embeds: Some(vec![embed]),
        username: settings.username,
        avatar_url: settings.avatar.clone(),
    });

    request(settings.url, msg, settings.broken_webhooks, settings.tx).await;
}

struct PasswordSettings {
    kind: Password,
    url: String,
    username: Option<String>,
    avatar: Option<String>,
    color: Option<u32>,
    footer_text: Option<String>,
    footer_image: Option<String>,
    timestamp: bool,
    store_name: String,
    store_url: String,
    store_logo: String,
    broken_webhooks: Arc<RwLock<Vec<String>>>,
    tx: Sender<Update>,
}

#[derive(PartialEq)]
enum Password {
    Up,
    Down,
}

// While this function currently isn't used anywhere, it has been tested
// and seems to be working. In the future, it could be used to allow
// users to view a preview of the embeds, which could be convenient when
// setting up the monitor using the command line in future versions of
// the program.
#[allow(dead_code)]
fn preview(msg: Arc<Message>) -> String {
    format!(
        "https://discohook.org/?data={}",
        encode_config(
            format!(
                "{{\"messages\":[{{\"data\":{}}}]}}",
                serde_json::to_string(&*msg).expect("Failed to serialize JSON.")
            ),
            URL_SAFE
        )
    )
}
