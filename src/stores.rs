// This module should use `config` to read `config.json`, then create a
// `stores` vector from the returned `Config` struct, containing all
// specified sites and their settings for the monitor to more easily and
// efficiently work with.

use crate::{alternative::Alternative as Alt, config, default, hidden, warning};
use std::sync::Arc;

pub fn get() -> Vec<Store> {
    let c = config::read();

    // this vector, which will then be passed to the `monitor::run()`
    // function, will be filled with one `Store` struct per site listed
    // in `config.json`, along with every event the user has selected.
    let mut stores: Vec<Store> = vec![];

    for site in c.sites {
        // A mutable vector is created for each event type
        let mut restock: Vec<Arc<Channel>> = vec![];
        let mut password_up: Vec<Arc<Channel>> = vec![];
        let mut password_down: Vec<Arc<Channel>> = vec![];

        // I'm checking "://" instead of "https://" and "http://"
        // it's briefer.
        let logo = if site.logo.contains("://") {
            site.logo
        } else {
            // This match block should include all the images saved in
            // the repository's logo folder.
            match site.logo.to_lowercase().chars().filter(|c| !c.is_whitespace()).collect::<String>().as_str() {
                "shopify" => "https://raw.githubusercontent.com/subreme/shopify-monitor/main/logos/shopify.png",

                // These sites should be listed alphabetically.
                "afew" => "https://raw.githubusercontent.com/subreme/shopify-monitor/main/logos/afew.jpg",
                "asphaltgold" => "https://raw.githubusercontent.com/subreme/shopify-monitor/main/logos/asphaltgold.jpg",
                "atmos" => "https://raw.githubusercontent.com/subreme/atmos-monitor/main/logos/atmos.jpg",
                "bodega" => "https://raw.githubusercontent.com/subreme/shopify-monitor/main/logos/bodega.png",
                "concepts" => "https://raw.githubusercontent.com/subreme/concepts-monitor/main/logos/concepts.jpg",
                "extrabutter" => "https://raw.githubusercontent.com/subreme/extrabutter-monitor/main/logos/extrabutter.jpg",
                "hanon" => "https://raw.githubusercontent.com/subreme/hanon-monitor/main/logos/hanon.jpg",
                "jimmyjazz" => "https://raw.githubusercontent.com/subreme/shopify-monitor/main/logos/jimmyjazz.jpg",
                "kith" => "https://raw.githubusercontent.com/subreme/shopify-monitor/main/logos/kith.jpg",
                "notre" => "https://raw.githubusercontent.com/subreme/shopify-monitor/main/logos/notre.jpg",
                "packer" => "https://raw.githubusercontent.com/subreme/shopify-monitor/main/logos/packer.jpg",
                "shoepalace" => "https://raw.githubusercontent.com/subreme/shopify-monitor/main/logos/shoepalace.jpg",
                "sneakerpolitics" => "https://raw.githubusercontent.com/subreme/shopify-monitor/main/logos/sneakerpolitics.jpg",
                "travisscott" | "cactusjack" => "https://raw.githubusercontent.com/subreme/shopify-monitor/main/logos/travisscott.jpg",
                "undefeated" => "https://raw.githubusercontent.com/subreme/shopify-monitor/main/logos/undefeated.jpg",
                "westnyc" => "https://raw.githubusercontent.com/subreme/shopify-monitor/main/logos/westnyc.jpg",

                // If the field doesn't contain a URL to an image or the
                // names of any "known" stores, the user will be warned
                // and Shopify's logo will be used as a replacement.
                _ => {
                    warning!("Invalid image for `{}`: `{}`!", site.name, site.logo);
                    default!("Using Shopify's logo instead...");

                    "https://raw.githubusercontent.com/subreme/shopify-monitor/main/logos/shopify.png"
                }
            }.into()
        };

        // `tokio::time::(interval)` panics if the duration is 0, so if
        // a non-zero duration wasn't specified, it must be set.
        let delay = {
            // This is the current minimum delay, in milliseconds.
            let mut minimum = 1u64;

            if let Some(time) = site.delay {
                if time > minimum {
                    minimum = time;
                }
            }
            minimum
        };

        // It would be better to use `&c.servers` instead of
        // `c.servers.clone()`, but I haven't implemented the required
        // traits for it yet, and have delayed this project's initial
        // release for too long for me to work on those now. This module
        // is only run when the program is initializing, so the slight
        // increase in memory usage isn't a concern.
        for server in c.servers.clone() {
            // After creating `Alt` I was posed with a dilemma related
            // to where I should place the logic to determine which
            // settings to use in each channel. Although the performance
            // impact of my choice should be minimal, and the code would
            // only run on monitor startup anyway, I considered all
            // options.

            // The choice would be mostly impacted by what the most
            // common user behavior is, as the more "nested" the logic
            // is placed, the greater the number of times it needs to be
            // repeated. The only problem is: the project's repository
            // hasn't been published yet.

            // If users were to usually individually configure each
            // event, it would be slightly more efficient to make all
            // checks insider the events `for` loop, but that seems
            // unlikely. The simplest way to configure the monitor is to
            // set shared settings when possible to avoid unnecessary
            // repetition. As a result, I decided to process the
            // settings as "outside" as possible, fragmenting the checks
            // to take place at all three "levels".

            // Alternatively, commonly "shared" settings, such as
            // `username` and `avatar`, could be partly processed
            // "outside", while other settings could be entirely checked
            // "inside", however I decided against this for
            // consistency's sake, and because the selected approach
            // allowed for the code to be essentially copied and pasted
            // between layers.

            // Another possible solution, in the future, could be to set
            // all default values here, then check for each event
            // directly if the value should be changed or not.

            // There is no need for `Option<bool>`s, as `None` would be
            // equivalent to the default boolean value anyway.

            let mut username: Option<String> = None;
            let mut avatar: Option<String> = None;
            let mut color: Option<String> = None;
            let mut sizes: bool = false;
            let mut thumbnail: bool = false;
            let mut image: bool = false;
            let mut footer_text: Option<String> = None;
            let mut footer_image: Option<String> = None;
            let mut timestamp: bool = false;

            if let Alt::Some(settings) = &server.settings {
                // Although in the example in `config.rs` I used `.is_some()`
                // and `.unwrap()`, i know what those methods contain, as I
                // wrote them, and using `if let` uses fewer steps.
                if let Alt::Some(value) = &settings.username {
                    username = Some(value.into())
                }

                if let Alt::Some(value) = &settings.avatar {
                    avatar = Some(value.into())
                }

                if let Alt::Some(value) = &settings.color {
                    color = Some(value.into())
                }

                if let Alt::Some(value) = settings.sizes {
                    sizes = value
                }

                if let Alt::Some(value) = settings.thumbnail {
                    thumbnail = value
                }

                if let Alt::Some(value) = settings.image {
                    image = value
                }

                if let Alt::Some(value) = &settings.footer_text {
                    footer_text = Some(value.into())
                }

                if let Alt::Some(value) = &settings.footer_image {
                    footer_image = Some(value.into())
                }

                if let Alt::Some(value) = settings.timestamp {
                    timestamp = value
                }
            }

            for channel in server.channels {
                if let Alt::Some(settings) = &channel.settings {
                    if let Alt::Some(value) = &settings.username {
                        username = Some(value.into())
                    } else if settings.username.is_null() {
                        username = None;
                    }

                    if let Alt::Some(value) = &settings.avatar {
                        avatar = Some(value.into())
                    } else if settings.avatar.is_null() {
                        avatar = None;
                    }

                    if let Alt::Some(value) = &settings.color {
                        color = Some(value.into())
                    } else if settings.color.is_null() {
                        color = None;
                    }

                    if let Alt::Some(value) = settings.sizes {
                        sizes = value
                    } else if settings.sizes.is_null() {
                        sizes = false;
                    }

                    if let Alt::Some(value) = settings.thumbnail {
                        thumbnail = value
                    } else if settings.thumbnail.is_null() {
                        thumbnail = false;
                    }

                    if let Alt::Some(value) = settings.image {
                        image = value
                    } else if settings.image.is_null() {
                        image = false;
                    }

                    if let Alt::Some(value) = &settings.footer_text {
                        footer_text = Some(value.into())
                    } else if settings.footer_text.is_null() {
                        footer_text = None;
                    }

                    if let Alt::Some(value) = &settings.footer_image {
                        footer_image = Some(value.into())
                    } else if settings.footer_image.is_null() {
                        footer_image = None;
                    }

                    if let Alt::Some(value) = settings.timestamp {
                        timestamp = value
                    } else if settings.timestamp.is_null() {
                        timestamp = false;
                    }
                } else if channel.settings.is_null() {
                    username = None;
                    avatar = None;
                    color = None;
                    sizes = false;
                    thumbnail = false;
                    image = false;
                    footer_text = None;
                    footer_image = None;
                    timestamp = false;
                }

                // Just to clarify, in this context `site` refers to the
                // website being monitored, as the program iterates
                // through each one and checks if it's referenced, as a
                // `store`, within a channel.
                for store in channel.sites.clone() {
                    if let Alt::Some(settings) = &store.settings {
                        if let Alt::Some(value) = &settings.username {
                            username = Some(value.into())
                        } else if settings.username.is_null() {
                            username = None;
                        }

                        if let Alt::Some(value) = &settings.avatar {
                            avatar = Some(value.into())
                        } else if settings.avatar.is_null() {
                            avatar = None;
                        }

                        if let Alt::Some(value) = &settings.color {
                            color = Some(value.into())
                        } else if settings.color.is_null() {
                            color = None;
                        }

                        if let Alt::Some(value) = settings.sizes {
                            sizes = value
                        } else if settings.sizes.is_null() {
                            sizes = false;
                        }

                        if let Alt::Some(value) = settings.thumbnail {
                            thumbnail = value
                        } else if settings.thumbnail.is_null() {
                            thumbnail = false;
                        }

                        if let Alt::Some(value) = settings.image {
                            image = value
                        } else if settings.image.is_null() {
                            image = false;
                        }

                        if let Alt::Some(value) = &settings.footer_text {
                            footer_text = Some(value.into())
                        } else if settings.footer_text.is_null() {
                            footer_text = None;
                        }

                        if let Alt::Some(value) = &settings.footer_image {
                            footer_image = Some(value.into())
                        } else if settings.footer_image.is_null() {
                            footer_image = None;
                        }

                        if let Alt::Some(value) = settings.timestamp {
                            timestamp = value
                        } else if settings.timestamp.is_null() {
                            timestamp = false;
                        }
                    } else if channel.settings.is_null() {
                        username = None;
                        avatar = None;
                        color = None;
                        sizes = false;
                        thumbnail = false;
                        image = false;
                        footer_text = None;
                        footer_image = None;
                        timestamp = false;
                    }

                    // Since every event is being checked and the
                    // channels are then saved in a `Vec`, the program
                    // will include duplicate channels if a user
                    // accidentally improperly configures the monitor.
                    // In the future, this could be prevented by using
                    // `HashMap`s with webhook URLs as keys, instead.
                    for event in store.events.clone() {
                        if let Alt::Some(settings) = &event.settings {
                            if let Alt::Some(value) = &settings.username {
                                username = Some(value.into())
                            } else if settings.username.is_null() {
                                username = None;
                            }

                            if let Alt::Some(value) = &settings.avatar {
                                avatar = Some(value.into())
                            } else if settings.avatar.is_null() {
                                avatar = None;
                            }

                            if let Alt::Some(value) = &settings.color {
                                color = Some(value.into())
                            } else if settings.color.is_null() {
                                color = None;
                            }

                            if let Alt::Some(value) = settings.sizes {
                                sizes = value
                            } else if settings.sizes.is_null() {
                                sizes = false;
                            }

                            if let Alt::Some(value) = settings.thumbnail {
                                thumbnail = value
                            } else if settings.thumbnail.is_null() {
                                thumbnail = false;
                            }

                            if let Alt::Some(value) = settings.image {
                                image = value
                            } else if settings.image.is_null() {
                                image = false;
                            }

                            if let Alt::Some(value) = &settings.footer_text {
                                footer_text = Some(value.into())
                            } else if settings.footer_text.is_null() {
                                footer_text = None;
                            }

                            if let Alt::Some(value) = &settings.footer_image {
                                footer_image = Some(value.into())
                            } else if settings.footer_image.is_null() {
                                footer_image = None;
                            }

                            if let Alt::Some(value) = settings.timestamp {
                                timestamp = value
                            } else if settings.timestamp.is_null() {
                                timestamp = false;
                            }
                        } else if channel.settings.is_null() {
                            username = None;
                            avatar = None;
                            color = None;
                            sizes = false;
                            thumbnail = false;
                            image = false;
                            footer_text = None;
                            footer_image = None;
                            timestamp = false;
                        }

                        // The `color()` function has been temporarily removed.

                        // // A webhook's embed color can be specified in two
                        // // places: within the `Event` itself, where it can
                        // // be individually customized, or in the `Server`'s
                        // // `ServerSettings`, whose value should be used if
                        // // one isn't specified for the `Event`.

                        // // Creating a function that's only called once [in
                        // // the code] and requiring so many parameters may
                        // // seem counter-intuitive, but after wasting some
                        // // time trying to properly assign values to the
                        // // `color` variable from within nested `if let`
                        // // statements (to no avail), I decided to use a
                        // // function, always using `return`, to "calm down"
                        // // the compiler.
                        // let color = color(
                        //     event.color.to_owned(),
                        //     server.settings.color.to_owned(),
                        //     server.name.to_owned(),
                        //     channel.name.to_owned(),
                        //     channel.id,
                        // );

                        let color = parse_color(color.to_owned());

                        // If the site being iterated through is
                        // mentioned in a a channel (one of all the ones
                        // also being iterated through), its values are collected.
                        if store.name == site.name {
                            let channel = Arc::new(Channel {
                                name: channel.name.to_owned(),
                                // id: channel.id.to_owned(),
                                url: channel.url.to_owned(),
                                settings: Settings {
                                    username: username.to_owned(),
                                    avatar: avatar.to_owned(),
                                    color,
                                    sizes,
                                    // atc: server.settings.atc,
                                    // price: server.settings.price,
                                    thumbnail,
                                    image,
                                    footer_text: footer_text.to_owned(),
                                    footer_image: footer_image.to_owned(),
                                    timestamp,
                                },
                            });

                            // This is no longer the case, as default
                            // values were removed.
                            // // There are only three possible values for
                            // // `restock`, `password_up`, and
                            // // `password_down` in an `Event` struct, as
                            // // they are optional and their type is
                            // // `Option<bool>`:
                            // // - Some(true)
                            // // - Some(false)
                            // // - None

                            // // Since `restock` has a default value
                            // // of`true`, this event should be included
                            // // if its value is either `Some(true)` or
                            // // `None`, two of the three options. This
                            // // check can therefore be more concisely
                            // // made by verifying that its value is NOT
                            // // `Some(false)`, the third kind.

                            // It is then added to the list (`Vec`) of
                            // channels that will receive a webhook
                            // notifying the occurrence of an event.

                            if event.restock == Some(true) {
                                restock.push(channel.clone());
                            }

                            // This is also no longer the case.
                            // // The other two event types default to
                            // // false, therefore the program only has to
                            // // check if their value is `Some(true)`.
                            if event.password_up == Some(true) {
                                password_up.push(channel.clone());
                            }

                            if event.password_down == Some(true) {
                                password_down.push(channel.clone());
                            }
                        }
                    }
                }
            }
        }

        // A site will only be monitored if it needs to be. If a store
        // is configured but no channel will receive its updates,
        // sending requests to the website is useless.
        if !restock.is_empty() || !password_up.is_empty() || !password_down.is_empty() {
            stores.push(Store {
                name: site.name.to_owned(),
                url: site.url.to_owned(),
                logo,
                delay,
                restock: Arc::new(restock),
                password_up: Arc::new(password_up),
                password_down: Arc::new(password_down),
            })
        }
    }
    stores
}

fn parse_color(color: Option<String>) -> Option<u32> {
    if let Some(code) = color {
        return Some(match code.to_lowercase().as_str() {
            "white" => 0xffffff,
            "black" => 0x000000,

            // These are the "light" (top row) Discord role colors
            "turquoise" => 0x1abc9c,
            "green" => 0x2ecc71,
            "blue" => 0x3498db,
            "purple" | "lilac" => 0x9b59b6,
            "pink" | "magenta" => 0xe91e63,
            "yellow" => 0xf1c40f,
            "orange" => 0xe67e22,
            "red" => 0xe74c3c,
            "light" | "lightgray" | "lightgrey" | "light gray" | "light grey" => 0x95a5a6,
            "gray" | "grey" | "dark" | "darkgray" | "darkgrey" | "dark gray" | "dark grey" => {
                0x607d8b
            }

            // The program will return `None` anyway if the code is
            // invalid.
            // // These are meant to help if someone uses this setting
            // // incorrectly (although, wouldn't that technically make it
            // // proper usage? :brain:)
            // // "null" | "none" | "no" => return None,
            _ => {
                if let Ok(val) = u32::from_str_radix(code.trim_start_matches('#'), 16) {
                    if val <= 0xFFFFFF {
                        return Some(val);
                    }
                    hidden!("Invalid Color (`{}`): Too Large!", code);
                } else {
                    hidden!("Invalid Color (`{}`): Not Hex!", code);
                }

                return None;
            }
        });
    }
    None
}

// This function has been temporarily removed due to the introduction of
// `Alt<T>`, as it requires changes. It will most likely be updated and
// "added back" in the future.

// // As explained above, this code was placed in a single-use function to
// // avoid bugs, and to allow for the process to be tested.
// pub fn color(
//     event_color: Option<String>,
//     server_color: Option<String>,
//     server_name: String,
//     channel_name: Option<String>,
//     channel_id: u64,
// ) -> Option<u32> {
//     if let Some(code) = event_color {
//         let code = code.trim();

//         if code.is_empty() {
//             return None;
//         }

//         if code.to_lowercase() == *"server" {
//             return color_server(server_color, server_name);
//         }

//         if let Ok(val) = u32::from_str_radix(code.trim_start_matches('#'), 16) {
//             if val <= 0xFFFFFF {
//                 return Some(val);
//             }
//         }

//         // This part will run if the event's color code
//         // was invalid.
//         hidden!(
//             "Invalid Color Code ({}) in {}'s {} channel!",
//             code,
//             server_name,
//             {
//                 if let Some(name) = channel_name {
//                     format!("{} ({})", name, channel_id)
//                 } else {
//                     format!("{}", channel_id)
//                 }
//             }
//         );
//         hidden!("Trying {}'s backup color...", server_name);

//         color_server(server_color, server_name)

//     // If a color isn't provided, the server's one should be used.
//     } else {
//         color_server(server_color, server_name)
//     }
// }

// // This function is used to replace repetitive segments of the `color()`
// // function above.
// fn color_server(server_color: Option<String>, server_name: String) -> Option<u32> {
//     if let Some(code) = server_color {
//         if let Ok(val) = u32::from_str_radix(code.trim_start_matches('#'), 16) {
//             Some(val)
//         } else {
//             hidden!("Invalid Color Code ({}) in {}!", code, server_name);
//             None
//         }
//     } else {
//         None
//     }
// }

pub struct Store {
    pub name: String,
    pub url: String,
    pub logo: String,

    // This field isn't optional, as a default value is set if one
    // wasn't configured.
    pub delay: u64,
    pub restock: Arc<Vec<Arc<Channel>>>,
    pub password_up: Arc<Vec<Arc<Channel>>>,
    pub password_down: Arc<Vec<Arc<Channel>>>,
}

pub struct Channel {
    pub name: String,
    // pub id: u64,
    pub url: String,
    // pub include: Option<Vec<String>>,
    // pub exclude: Option<Vec<String>>,
    // pub proxies: Option<Vec<String>>,
    pub settings: Settings,
}

pub struct Settings {
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub color: Option<u32>,
    pub sizes: bool,
    // pub atc: Option<bool>,
    // pub price: Option<bool>,
    pub thumbnail: bool,
    pub image: bool,
    pub footer_text: Option<String>,
    pub footer_image: Option<String>,
    pub timestamp: bool,
}
