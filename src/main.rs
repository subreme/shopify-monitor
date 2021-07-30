mod alternative;
mod config;
mod log;
mod message;
mod monitor;
mod products;
mod stores;
mod tests;
mod webhook;

use colored::*;

#[tokio::main]
async fn main() {
    hidden!("Starting Program...");

    // This weird-looking string is an "Ascii-Art Font" representation
    // of "Shopify Monitor", with "Shopify" printed green, using the
    // `colored` crate, to somewhat resemble the company logo.
    println!(
        "  {}         __  __             _ _\n {}       |  \\/  |           (_) |\n{}  | \\  / | ___  _ __  _| |_ ___  _ __\n {} | |\\/| |/ _ \\| '_ \\| | __/ _ \\| '__|\n {} | |  | | (_) | | | | | || (_) | |\n{} |_|  |_|\\___/|_| |_|_|\\__\\___/|_|\n                   {}\n                   {}\n",
        "_____ _                 _  __".green(),
        "/ ____| |               (_)/ _|".green(),
        "| (___ | |__   ___  _ __  _| |_ _   _".green(),
        "\\___ \\| '_ \\ / _ \\| '_ \\| |  _| | | |".green(),
        "____) | | | | (_) | |_) | | | | |_| |".green(),
        "|_____/|_| |_|\\___/| .__/|_|_|  \\__, |".green(),
        "| |           __/ |".green(),
        "|_|          |___/".green()
    );

    // The output will look like this:
    //   _____ _                 _  __         __  __             _ _
    //  / ____| |               (_)/ _|       |  \/  |           (_) |
    // | (___ | |__   ___  _ __  _| |_ _   _  | \  / | ___  _ __  _| |_ ___  _ __
    //  \___ \| '_ \ / _ \| '_ \| |  _| | | | | |\/| |/ _ \| '_ \| | __/ _ \| '__|
    //  ____) | | | | (_) | |_) | | | | |_| | | |  | | (_) | | | | | || (_) | |
    // |_____/|_| |_|\___/| .__/|_|_|  \__, | |_|  |_|\___/|_| |_|_|\__\___/|_|
    //                    | |           __/ |
    //                    |_|          |___/

    important!("LOADING SETTINGS");

    // This function calls for `config.json` to be loaded by `config`,
    // then be deserialized and sent over to `stores` to be used the
    // generate the settings for each monitored website.
    let settings = stores::get();

    important!("STARTING MONITOR");

    // Once the `settings` are returned, the monitor can start running.
    monitor::run(settings).await;
}
