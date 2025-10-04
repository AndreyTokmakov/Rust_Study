

mod demo_one
{
    use std::time::{SystemTime, UNIX_EPOCH};

    fn log(level: &str, msg: &str)
    {
        // Get current time in seconds since epoch
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();

        // Format [LEVEL] [timestamp] Message
        println!("[{:<5}] [{}] {}",  level, now.as_secs(), msg);
    }

    pub fn test()
    {
        log("INFO", "Application started");
        log("WARN", "Low disk space");
        log("ERROR", "Failed to connect to database");
    }
}


mod demo_two
{
    use chrono::{DateTime, Local};
    use chrono::format::{DelayedFormat, StrftimeItems};

    fn log(level: &str, msg: &str)
    {
        let now: DateTime<Local> = Local::now();
        let timestamp: DelayedFormat <StrftimeItems> = now.format("%Y-%m-%d %H:%M:%S");
        println!("[{:<5}] [{}] {}", level, timestamp, msg);
    }

    pub fn test()
    {
        log("INFO", "Server started");
        log("DEBUG", "Listening on port 8080");
        log("ERROR", "Connection lost");
    }
}

mod logger_with_macro
{
    use chrono::{DateTime, Local};
    use chrono::format::{DelayedFormat, StrftimeItems};

    fn log(level: &str, msg: &str)
    {
        let now: DateTime<Local> = Local::now();
        let timestamp: DelayedFormat <StrftimeItems> = now.format("%Y-%m-%d %H:%M:%S");
        println!("[{:<5}] [{}] {}", level, timestamp, msg);
    }

    macro_rules! info {
    ($($arg:tt)*) => {
            log("INFO", &format!($($arg)*));
        };
    }

    macro_rules! warn {
    ($($arg:tt)*) => {
            log("WARN", &format!($($arg)*));
        };
    }

    macro_rules! error {
    ($($arg:tt)*) => {
            log("ERROR", &format!($($arg)*));
        };
    }

    pub fn test()
    {
        info!("Starting server on port {}", 8080);
        warn!("Low memory: {} MB left", 120);
        error!("Failed to read file: {}", "/etc/config.json");

        // [INFO ] [2025-10-04 12:32:54] Starting server on port 8080
        // [WARN ] [2025-10-04 12:32:54] Low memory: 120 MB left
        // [ERROR] [2025-10-04 12:32:54] Failed to read file: /etc/config.json
    }
}

mod logger_with_macro_stderr_stdout
{
    use chrono::{DateTime, Local};
    use std::io::{self, Write};
    use chrono::format::{DelayedFormat, StrftimeItems};

    fn log_to(level: &str, msg: &str, use_stderr: bool)
    {
        let now: DateTime<Local> = Local::now();
        let timestamp: DelayedFormat <StrftimeItems> = now.format("%Y-%m-%d %H:%M:%S");
        let output: String = format!("[{:<5}] [{}] {}\n", level, timestamp, msg);

        if use_stderr {
            io::stderr().write_all(output.as_bytes()).unwrap();
        } else {
            io::stdout().write_all(output.as_bytes()).unwrap();
        }
    }

    macro_rules! info {
        ($($arg:tt)*) => {
            log_to("INFO", &format!($($arg)*), false);
        };
    }

    macro_rules! warn {
        ($($arg:tt)*) => {
            log_to("WARN", &format!($($arg)*), true);
        };
    }

    macro_rules! error {
        ($($arg:tt)*) => {
            log_to("ERROR", &format!($($arg)*), true);
        };
    }

    pub fn test()
    {
        info!("Starting server on port {}", 8080);
        warn!("Low memory: {} MB left", 120);
        error!("Failed to read file: {}", "/etc/config.json");

        // [INFO ] [2025-10-04 12:32:54] Starting server on port 8080
        // [WARN ] [2025-10-04 12:32:54] Low memory: 120 MB left
        // [ERROR] [2025-10-04 12:32:54] Failed to read file: /etc/config.json
    }
}

mod colored_output
{
    use chrono::{DateTime, Local};
    use chrono::format::{DelayedFormat, StrftimeItems};

    fn log(level: &str, msg: &str)
    {
        let now: DateTime<Local> = Local::now();
        let timestamp: DelayedFormat <StrftimeItems> = now.format("%Y-%m-%d %H:%M:%S");

        let level_str = match level {
            "INFO"  => format!("\x1b[32m{:<5}\x1b[0m", level),  // green
            "WARN"  => format!("\x1b[33m{:<5}\x1b[0m", level),  // yellow
            "ERROR" => format!("\x1b[31m{:<5}\x1b[0m", level),  // red
            _ => level.to_string(),
        };

        println!("[{}] [{}] {}", level_str, timestamp, msg);
    }

    pub fn test()
    {
        log("INFO", "Server started");
        log("WARN", "Disk almost full");
        log("ERROR", "Connection failed");

        // [INFO ] [2025-10-04 12:38:42] Server started
        // [WARN ] [2025-10-04 12:38:42] Disk almost full
        // [ERROR] [2025-10-04 12:38:42] Connection failed
    }
}


pub fn test_all()
{
    // demo_one::test();
    // demo_two::test();
    // logger_with_macro::test();
    // logger_with_macro_stderr_stdout::test();
    colored_output::test();
}