// This module should allow the `stores` module to read `config.json`
// and have a `Config` struct returned, so that it can be processed
// further in there.

use crate::{alternative::Alternative as Alt, default, error, hidden, success, warning};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io, process, vec::IntoIter};

// This function is used to get the deserialized values saved in
// `config.json` in order for the program to know what to do.
pub fn read() -> Config {
    // At first, the program assumes it's being run by a regular user
    // and doesn't mention the possible existence of
    // `config.private.json`.

    // This file is intended to be used by developers, as the repository's
    // `.gitignore` doesn't include `config.json`. By configuring their
    // own settings in the private file, leaving the "public" one with
    // its example settings, they can ensure that they don't
    // accidentally publish their webhook URLs and the integrity of the
    // example configuration isn't compromised.

    hidden!("Loading `config.private.json`...");
    default!("Loading config file...");

    if let Ok(config) = fs::read_to_string("config.private.json") {
        hidden!("Reading `private.config.json`...");

        // The program only refers to the private config file as such if
        //the directory it's in contains it.
        default!("Reading private config file...");

        let json = serde_json::from_str(config.as_str());

        if let Ok(value) = json {
            success!("Successfully parsed settings!");
            return value;
        } else if let Err(error) = json {
            hidden!("Failed to parse `config.private.json`: {}", error);
        }

        warning!("Invalid private config file!");
        default!("Trying again with `config.json`...");
    };

    // If the user didn't have a `config.private.json` file, the program
    // moves on to `config.json` like nothing happened, in order not to
    // confuse the user.

    // This function's return value is assigned to a variable instead of
    // being directly included using `if let`, as the `Error` type has
    // to be checked in order to determine whether `config.json` exists
    // or if it has to be created.
    let file = fs::read_to_string("config.json");

    if let Ok(config) = &file {
        default!("Reading config file...");

        let json = serde_json::from_str(config.as_str());

        if let Ok(value) = json {
            success!("Successfully parsed settings!");
            return value;
        } else if let Err(error) = json {
            hidden!("Failed to parse `config.json`: {}", error);
        }

        error!("Invalid config file!");
    }

    // If the function fails, it's error is extracted and its
    // `ErrorKind` is checked.
    if let Err(error) = file {
        // If `config.json` wasn't found, meaning that the it doesn't
        // exist, the file is created.
        if error.kind() == io::ErrorKind::NotFound {
            error!("`config.json` not found!");
            default!("Creating `config.json`...");

            let write = fs::write("config.json", "");

            // Creating the file could also fail, and since this is Rust
            // we have to account for that too.
            if write.is_ok() {
                success!("Created `config.json`!");
            } else {
                error!("Failed to create `config.json`.");
            }
        }
    };

    // Regardless of what happens, the project's guide is linked and
    // the user is invited to read the instructions in order to fix
    // the issue, then the program is closed.
    suggest_instructions();
    process::exit(0);
}

// In the future, this function will be used to update the settings,
// saving new config values if modified or selected through means other
// than directing modifying `config.json`.
#[allow(dead_code)]
pub fn write(config: &Config) {
    // The `config` values need to be serialized to JSON in order to be
    // exported. Although it could be minified to use slightly less
    // storage, the JSON is "beautified" so that it's easier to read.
    let data = serde_json::to_string_pretty(config);
    if let Ok(text) = data {
        // Once the values have been converted to text, they can finally
        // be written to `config.json`.
        if fs::write("config.array.json", text).is_ok() {
            success!("Saved settings to `config.json`!");
        } else {
            warning!("Failed to write to `config.json`.");
        }

    // Serialization should never fail, but in the rare case it does,
    // the error should be logged.
    } else {
        warning!("Failed to serialize to JSON.");
    }
}

// This brief function is only used twice, as it simply provides a link
// to the project's GitHub repository where the user can find the
// documentation to help properly configure the monitor.
fn suggest_instructions() {
    default!("Please follow the instructions on https://github.com/subreme/shopify-monitor to complete the configuration process.");
    error!("Press `Enter` to close the program...");

    // The program waits for the user to press the `Enter` key so that
    // there's enough time to read the error messages, before discarding
    // the new input and terminating the program.
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // The function could also call `process::exit()`, as it is repeated
    // after both of its instances, however the compiler wouldn't
    // recognize its presence outside of of `read()`, therefore it
    // wasn't included.
}

// The `Config` struct is defined here, and the `derive` macro allows
// `serde` to serialize and deserialize the contents of `config.json`
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub sites: VecMap<Site, SiteHM>,
    pub servers: VecMap<Server, ServerHM>,

    // This field is optional, as proxies aren't required for the
    // monitor to run.

    // For all practical use cases, the type of this fields allows it to
    // be structured in one of these two ways:

    // "proxies": [
    //     {
    //         "name": "foo",
    //         "proxies": []
    //     },
    //     {
    //         "name": "bar",
    //         "proxies": []
    //     }
    // ]

    // "proxies": {
    //     "foo": [],
    //     "foo": []
    // }

    // While the types allow for different structures too, these are the
    // two intended and recommended versions.
    pub proxies: Option<VecMap<ProxyList, VecMap<String, String>>>,
}

// While on a "quest" to create the best possible config file structure
// for this project, having used `Vec<T>`s everywhere, I found a old
// JavaScript monitor I had written which used the equivalent to
// `HashMap<String, T>`, and started wondering which option was best. I
// asked several people what they preferred, but I couldn't reach a
// consensus, so I decided to try and support both formats.

// After trying to replicate the examples described in in these two
// resources, but failing, I decided that a simpler solution would be
// better (for now), so I decided to use an enum.

// https://stackoverflow.com/questions/59607683/what-is-the-best-way-to-convert-a-vector-of-tuples-into-a-hashmap-with-a-value-o
// https://serde.rs/string-or-struct.html

// It takes two type parameters, assuming that the HashMap's key will be
// a string, because all JSON keys must be strings.

// When a second struct type is needed, usually because the first
// (`Vec`) one includes a field that corresponds to the key to the
// second (`HashMap`) one, the names of the two structs should be
// formatted this way: `VecMap<Foo, FooHM>`.

// Some fields only need one type (`T == U`), however since the addition
// of this type may already confuse possible contributors, I decided not
// to add yet another enum, and some instances of the enum will have to
// be annotated as `VecMap<Foo, Foo>`.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum VecMap<T, U> {
    Vec(Vec<T>),
    Map(HashMap<String, U>),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Site {
    pub name: String,
    pub url: String,
    pub logo: String,

    // While a `u64` type seems unnecessarily large for a value that
    // needs to hold a monitor's millisecond delay, it's the one used by
    // `tokio::time::interval()`, and using multiple conversions when
    // doing checks in `store.rs` was annoying.

    // Replacing all instances of `Option<T>` with `Alt<T>` is
    // unnecessary, as its benefits only apply to cases where a `null`
    // value is significant
    pub delay: Option<u64>,
}

// For a `VecMap<T, U>` to work, a second type `U` must be defined,
// where the "identifier", usually being the `name` field, should be
// removed from the struct as it will serve as the `HashMap`'s key.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SiteHM {
    pub url: String,
    pub logo: String,
    pub delay: Option<u64>,
}

// Implementing `IntoIterator` for each `VecMap` type used is easier
// than modifying the `stores` module's code to handle both variants,
// and allows me to find bugs more easily. Please note that it's the
// first time I try this, so please let me know if there's a better
// approach to do this.
impl IntoIterator for VecMap<Site, SiteHM> {
    type Item = Site;
    type IntoIter = IntoIter<Site>;

    fn into_iter(self) -> Self::IntoIter {
        if let VecMap::Vec(sites) = self {
            sites.into_iter()
        } else if let VecMap::Map(sitehms) = self {
            let mut sites = Vec::with_capacity(sitehms.len());

            for (name, sitehm) in sitehms {
                sites.push(Site {
                    name,
                    url: sitehm.url,
                    logo: sitehm.logo,
                    delay: sitehm.delay,
                });
            }

            sites.into_iter()

        // The compiler requires me to include this else block.
        } else {
            vec![].into_iter()
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Server {
    // If the user chooses to use a `Vec` (or rather, an array) to list
    // the servers, the server names don't have to be unique, as the
    // program "doesn't care". If they use a `HashMap<ServerHM>`,
    // however, the name is required to be unique, as a `Struct`
    // (object) can't contain two fields with the same name. Since the
    // name is only used for logs, additional characters can be added,
    // such as descriptions between parentheses, to circumvent this.
    pub name: String,

    // One of the next comments, on top of `Settings`'s definition, will
    // explain why this is of type `Alternative` instead of `Option`.
    #[serde(default)]
    pub settings: Alt<Settings>,

    // Channels are required, as there's no point in configuring a
    // server if the monitor can't send any webhooks to it. Of course,
    // if a server is in the process of being set up, an empty vector
    // (or "array", in JSON terms) is acceptable.

    // Although I see no reason for the user to do so, I decided that
    // every `Vec` within the `Config` struct should be replaced with a
    // `VecMap`, as, although it's often not necessary, it could help
    // the user organize their config file more easily.
    pub channels: VecMap<Channel, ChannelHM>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServerHM {
    #[serde(default)]
    pub settings: Alt<Settings>,
    pub channels: VecMap<Channel, ChannelHM>,
}

impl IntoIterator for VecMap<Server, ServerHM> {
    type Item = Server;
    type IntoIter = IntoIter<Server>;

    fn into_iter(self) -> Self::IntoIter {
        if let VecMap::Vec(servers) = self {
            servers.into_iter()
        } else if let VecMap::Map(serverhms) = self {
            let mut servers = Vec::with_capacity(serverhms.len());

            for (name, serverhm) in serverhms {
                servers.push(Server {
                    name,
                    channels: serverhm.channels,
                    settings: serverhm.settings,
                });
            }

            servers.into_iter()
        } else {
            vec![].into_iter()
        }
    }
}

// Every `Server`, `Channel`, `Store` (defined below, not to be confused
// with `Site`), and `Event` contains a `Settings` struct, so in order
// to properly determine which `Setting`'s value should be used, clear
// rules have to be set (and documented in the README).

// The settings hierarchy goes from specific to general, as if set, the
// most "focused" individual setting will be used. This means that, if
// included, a value set from within an `Event` will take precedence
// over one set in the `Store` it is for, which will overturn the
// `Channel` that contains it, which in turn will override a value set
// in the `Server` it's in. That's a mouthful, so let's see how that
// works.

// This conditional statement, determining the value of the imaginary
// setting `foo`, might help some users visualize the process:

// let foo: Option<String> = if event.settings.foo.is_some() {
//     event.settings.foo.to_option()
// } else if event.settings.foo.is_null() {
//     None
// } else /*if event.settings.foo.is_none()*/ {
//     if store.settings.foo.is_some() {
//         store.settings.foo.to_option()
//     } else if store.settings.foo.is_null() {
//         None
//     } else /*if store.settings.foo.is_none()*/ {
//         if channel.settings.foo.is_some() {
//             channel.settings.foo.to_option()
//         } else if channel.settings.foo.is_null() {
//             None
//         } else /*if channel.settings.foo.is_none()*/ {
//             if server.settings.foo.is_some() {
//               server.settings.foo.to_option()
//           } else if server.settings.foo.is_null() {
//               None
//           } else /*if server.settings.foo.is_none()*/ {
//               // Whatever the default value is.
//               "bar".into()
//           }
//         }
//     }
// };

// Although this isn't how the program's logic is written, and Clippy
// would complain about the `if ` statements being collapsible and some
// `else` statements being unnecessary (or some people would prefer to
// use `if let` etc.), this should allow the reader to easily follow the
// program's logic.

// A couple things stand out, most notably two implementations/methods:
// `to_option()` and `is_null()`. The former is necessary because foo's
// type is, as expected, not `Option`. The latter, on the other hand,
// does something that can't be done with `Option`: it checks if the
// [JSON] value is null. This is important, as `Option::Null` can
// represent both a missing JSON field and a `null` value, but can't
// distinguish between the two.

// To solve this issue, `crate::alternative` contains a similar enum,
// `Alternative`, which was renamed (or rather, abbreviated) to `Alt`
// for convenience. It contains three variants: `Some(T)`, which works
// in the same way as `Option::Sum(T)`, `Null`, which represents a
// `null` value, and `None`, which in this "alternative" type only
// represents missing (as in, not set or included) values.

// Before `Alt`'s creation, only `Server`s included `Settings`, and its
// values were all `Option`s. While the enum is very useful, it proved
// not to be sufficient for the logic in the code block above to be
// implemented when settings were added to `Channel`s and `Event`s.

// Even through `Alt` is very powerful, it's not necessary for the
// monitor itself to function, and should only be viewed as an
// "abstraction" of sorts, processed in the `stores.rs` module, in order
// to form the correct `Option` values that will be passed to the
// monitor's unchanged functions.

// Why are the settings fields also `Alt`s? Because this allows for the
// "higher-level" settings to be used for all values if they aren't
// included (achieving the same effect as an empty `struct`), and for
// only "default" values to be used if they're set to `null`.

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Settings {
    #[serde(default)]
    pub username: Alt<String>,
    #[serde(default)]
    pub avatar: Alt<String>,
    #[serde(default)]
    pub color: Alt<String>,
    #[serde(default)]
    pub sizes: Alt<bool>,

    // This toggle was removed, as I was unable to find a way to form
    // the URL to a product's variant, so the only unique link to each
    // variant is the Add To Cart one (which is therefore always
    // included).
    // pub atc: Option<bool>,

    // Although I was planning to include the option to select whether
    // to include the item price or not, I decided to "strip users of
    // this power" as I couldn't figure out a way to make the embeds
    // look balanced if the brand was included but the price wasn't.
    // pub price: Option<bool>,
    #[serde(default)]
    pub thumbnail: Alt<bool>,
    #[serde(default)]
    pub image: Alt<bool>,
    #[serde(default)]
    pub footer_text: Alt<String>,
    #[serde(default)]
    pub footer_image: Alt<String>,
    #[serde(default)]
    pub timestamp: Alt<bool>,

    // This field's type is `usize` as it's the same one use for the
    // length of `Vec`s, which this value is compared to.
    pub minimum: Alt<usize>,

    // For this field, a `HashMap` could help keep track of what each
    // "keyword group" is targeting, if that helps.
    #[serde(default)]
    pub keywords: Alt<VecMap<Keyword, Keyword>>,
}

// I was planning on calling this `Keywords`, as each one of these
// `struct`s actually contains multiple keyword, however I am fairly
// confident this this would have been the only type in this project
// with a plural name, so that would have been inconsistent (and would
// have "impeded me" from writing "for keyword in keywords", which would
// be weird).
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Keyword {
    // These fields are also `Alt`s, so that they can be set to `null`
    // to ignore the "higher level" ones set for a wider scope. Even
    // this, though, isn't enough to allow for complete customization
    // with little repetition.  they haven't been implemented yet, I am
    // planning to instead use `Option`s and an additional `combine`
    // boolean field.

    // Using `Alt` for `include` and `exclude`, a null value could be
    // used to ignore the keywords selected for a wider context, but if
    // keywords are allowed to "stack" an issue would arise: a value
    // can't be null and hold another value at the same time.

    // By adding `combine`, users can set its value to false and choose
    // keywords that won't be combined with "wider scope" ones, so that
    // they can set common "shared" keywords without having to repeat
    // them in every event's settings.

    // As a result, these two example are equivalent:

    // // Example 1:
    // "keywords": null,

    // // Example 2:
    // "keywords": [
    //     {
    //         "include": null,
    //         "exclude": null
    //     }
    // ],

    // // Example 3:
    // "keywords": [
    //     {
    //         "combine": false
    //     }
    // ],

    // Not including the field would not be equivalent if any of the
    // four settings relating to the event have a value.

    // Please note that `crate::stores` will convert this struct to a
    // different `Keyword` type, as `combine` and the use of `Alt` is
    // only necessary for pre-processing.

    // The use of `VecMap` is unnecessary, as they don't need a `name`
    // field, but it was used anyway to allow for more flexibility. If a
    // `HashMap` is used, its key will be ignored.

    // This is mainly because I don't want to deal with a "washed" user
    // complaining that he can't use a `HashMap` here, although I said
    // that any array could be replaced with an object in the README. I
    // can't imagine any scenario where someone would need to use a
    // `HashMap` here, but oh well.
    #[serde(default)]
    pub include: Alt<VecMap<String, String>>,
    #[serde(default)]
    pub exclude: Alt<VecMap<String, String>>,

    // This doesn't need to be an `Alt` as if it's null it will be given
    // the default value of `true` anyway. It's the only setting which
    // defaults to true, as users are encouraged to "stack" settings.
    pub combine: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Channel {
    pub name: String,

    // This struct used to contain an `id`, with the reasoning being
    // that many servers using this monitor might want to send webhooks
    // to identically named channels, however the `name` is only used
    // for logging and debugging purposes, and the ID wasn't ever used,
    // so it was removed.
    // pub id: u64,
    pub url: String,
    #[serde(default)]
    pub settings: Alt<Settings>,

    // By having the field names correspond to a site, it's impossible
    // for users to accidentally configure the same site twice for the
    // same channel.

    // The field name is `sites` and not `stores` as the former is the
    // term used to refer to the "stores" in the the rest of the config
    // file, and it's best to avoid confusion. After all, most users
    // won't be event aware that the type used within this field is
    // called `Store`.
    pub sites: VecMap<Store, StoreHM>,
}

// This type was trickier to create, as multiple channels often share
// the same name, which isn't allowed for HashMap keys, and while the
// `id`s are unique, JSON keys must be Strings.

//
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ChannelHM {
    // This field was removed. The original idea was that the value
    // would be used instead of the key to it as the channel name,
    // however I decided against it. Users are still allowed to include
    // they field if they choose to, but its value will be ignored.

    // // While this alternative struct could be removed entirely if the
    // // `name` field were made optional in the original version, as it is
    // // here, I decided to require it instead, so that debugging (and
    // // configuring the monitor) doesn't become ridiculously hard.

    // // In this struct, users all allowed to include a `name` field in
    // // the struct contained by a `HashMap` if they want to, bypassing
    // // the limitation cause by the type not allowing duplicate keys.
    // pub name: Option<String>,
    pub url: String,
    #[serde(default)]
    pub settings: Alt<Settings>,
    pub sites: VecMap<Store, StoreHM>,
}

impl IntoIterator for VecMap<Channel, ChannelHM> {
    type Item = Channel;
    type IntoIter = IntoIter<Channel>;

    fn into_iter(self) -> Self::IntoIter {
        if let VecMap::Vec(channels) = self {
            channels.into_iter()
        } else if let VecMap::Map(channelhms) = self {
            let mut channels = Vec::with_capacity(channelhms.len());

            for (name, channelhm) in channelhms {
                channels.push(Channel {
                    name,
                    url: channelhm.url,
                    settings: channelhm.settings,
                    sites: channelhm.sites,
                });
            }

            channels.into_iter()
        } else {
            vec![].into_iter()
        }
    }
}

// Just like with `Keyword`, `crate::stores` has an identically named
// struct, but this is not an issue as this type is only meant to be used
// from within this module.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Store {
    pub name: String,
    #[serde(default)]
    pub settings: Alt<Settings>,

    // If they choose so, users can "name" each event by using a
    // `HashMap`. Since this value is never checked, the `Event` struct
    // doesn't contain a `name` field, allowing it to be included for
    // both `VecMap` variants without being read.
    pub events: VecMap<Event, Event>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct StoreHM {
    #[serde(default)]
    pub settings: Alt<Settings>,
    pub events: VecMap<Event, Event>,
}

impl IntoIterator for VecMap<Store, StoreHM> {
    type Item = Store;
    type IntoIter = IntoIter<Store>;

    fn into_iter(self) -> Self::IntoIter {
        if let VecMap::Vec(stores) = self {
            stores.into_iter()
        } else if let VecMap::Map(storehms) = self {
            let mut stores = Vec::with_capacity(storehms.len());

            for (name, storehm) in storehms {
                stores.push(Store {
                    name,
                    settings: storehm.settings,
                    events: storehm.events,
                });
            }

            stores.into_iter()
        } else {
            vec![].into_iter()
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Event {
    #[serde(default)]
    pub settings: Alt<Settings>,
    pub restock: Option<bool>,
    pub password_up: Option<bool>,
    pub password_down: Option<bool>,
}

impl IntoIterator for VecMap<Event, Event> {
    type Item = Event;
    type IntoIter = IntoIter<Event>;

    fn into_iter(self) -> Self::IntoIter {
        if let VecMap::Vec(events) = self {
            events.into_iter()
        } else if let VecMap::Map(eventhms) = self {
            // events.values().collect::<Vec<Event>>().into_iter()

            let mut events = Vec::with_capacity(eventhms.len());

            for (_, eventhm) in eventhms {
                events.push(eventhm);
            }

            events.into_iter()
        } else {
            vec![].into_iter()
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProxyList {
    pub name: String,

    // Once again, the use of a `VecMap` here is completely unnecessary,
    // and it was only used so that in the unlikely scenario a user
    // chooses to use the program's "feature" of supporting a `HashMap`
    // wherever a `Vec` is allowed to the test, they'll find that there
    // are no exceptions.
    pub proxies: VecMap<String, String>,
}
