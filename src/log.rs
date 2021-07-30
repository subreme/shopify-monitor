// This module should include simple macros to print colored and
// timestamped logs to the terminal.

// The file contains a procedural macro (`#[cfg(not(test))]`) and a
// second declaration of the `file!()` macro, as I attempted to prevent
// any logs from being saved to a file during tests, before giving up as
// it caused several bugs in the rest of the program.

// This macro shouldn't be used directly, but rather called by this
// file's other macros in order to format their arguments and print them
// to the terminal. Its only current action is to add a timestamp and
// call `println!()`, however in the future it may also be used to print
// the text over multiple lines if it's over a certain length, etc.
#[macro_export]
macro_rules! log {
    ($msg:expr) => {
        // `%F` is equivalent to `%Y-%m-%d`, while `%T` is the same as
        // `%H:%M:%S`. `%.3f`, instead, represents milliseconds, as it
        // only considers the first three fractional seconds digits. As a
        // result, all log messages will begin with a timestamp in the
        // following format: `[YYYY-MM-DD hh:mm:ss.mmm]`.
        println!("[{}] {}", chrono::Local::now().format("%F %T%.3f"), $msg);
    };
}

// This macro also shouldn't be called outside this file, as it's
// implemented in the other macros to save logs to a file.
#[macro_export]
/*
// It should only save logs if it isn't called from during tests, as
// those logs would not be useful and could be confusing.
#[cfg(not(test))]
*/
macro_rules! file {
    // While they weren't included for `log!()`, the parentheses around
    // the curly brackets are necessary in this macro as the `Write`
    // trait, which must be imported for `write!()` to work, and may or
    // may not be already in scope when the macro is called. If they
    // weren't included and the macro were invoked in a scope where the
    // trait was already declared, it would be imported twice, causing
    // the code not to compile.
    ($($arg:tt)*) => ({
        // Traits must be explicitly imported using `use`.
        use std::io::Write;

        if let Ok(mut file) = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("shopify-monitor.log")
        {
            let _ = std::write!(
                file,
                "[{}] {}\n",
                chrono::Local::now().format("%F %T%.3f"),
                std::format_args!($($arg)*)
            );
        }
    });
}

/*
// While a test is running, it shouldn't do anything.
#[macro_export]
#[cfg(test)]
macro_rules! file {
    ($($arg:tt)*) => {};
}
*/

// The other macros take a string literal and some optional parameters,
// allowing them to be called in the same way as `println!()`. Its
// arguments are then styled differently in each macro, as each type of
// log is assigned a matching color.

// The `default!()` macro is the "general" type of log, simply printing
// white text, and is therefore suitable for most generic messages.
#[macro_export]
macro_rules! default {
    ($($arg:tt)*) => {
        // From now on I will worship this `.to_string()`, as after half
        // an hour of trying to solve a "future cannot be sent between
        // threads safely" error in monitors.rs, I tried adding it and
        // it surprisingly solved it. I hadn't noticed its absence as I
        // all the macros I had called from within Tokio Tasks contained
        // it, but as soon as I tried calling it, all hell broke lose
        // and the program stopped working. This little function, whose
        // absence hadn't caused any issues until now, is my new hero.
        let msg = std::format_args!($($arg)*).to_string();
        crate::log!(msg);
        crate::file!("[DEFAULT] {}", msg);
    };
}

// This one, on the other hand, colors the text green, and is suitable
// for positive updates, recording the successful completion of an
// operation. Exclamation marks are encouraged.
#[macro_export]
macro_rules! success {
    // All log macros producing stylized (colored) output need to import
    // the `Colored` trait, and are therefore also wrapped in parentheses.
    ($($arg:tt)*) => ({
        use colored::Colorize;

        let msg = std::format_args!($($arg)*).to_string();
        crate::log!(msg.green());
        crate::file!("[SUCCESS] {}", msg);
    });
}

// Displayed in yellow, these messages should cover small errors that do
// not cause major problems, forcing the program to close.
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => ({
        use colored::Colorize;

        let msg = std::format_args!($($arg)*).to_string();
        crate::log!(msg.yellow());
        crate::file!("[WARNING] {}", msg);
    });
}

// These red messages, on the other hand, are for significant, often
// unrecoverable errors. Although they don't have to be exclusively used
// for fatal errors, `warning()` is more appropriate for minor problems.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        use colored::Colorize;

        let msg = std::format_args!($($arg)*).to_string();
        crate::log!(msg.red());
        crate::file!("[ERROR] {}", msg);
    });
}

// This macro should be rarely used, as it's mainly intended to
// signal the start of certain processes, such as loading the setting,
// starting the monitor, and so on. The use of all-caps and no
// punctuation is suggested.
#[macro_export]
macro_rules! important {
    ($($arg:tt)*) => ({
        use colored::Colorize;

        let msg = std::format_args!($($arg)*).to_string();
        crate::log!(msg.blue());
        crate::file!("[IMPORTANT] {}", msg);
    });
}

// These messages will not be displayed in the console, but only saved
// to a file in a non-distracting way. They can therefore be used much
// more frequently, possibly allowing for better debugging.
#[macro_export]
macro_rules! hidden {
    // As expected, this macro does not include parentheses, as it
    // doesn't include import statements.
    ($($arg:tt)*) => {
        crate::file!("[HIDDEN] {}", std::format_args!($($arg)*));
    };
}
