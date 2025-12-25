

mod tracing_basic
{
    use tracing::{info, debug, warn, error, span, Level};
    use tracing_subscriber::fmt::format::FmtSpan;

    pub fn simple_logs()
    {
        // Initialize a global subscriber for logging
        tracing_subscriber::fmt()
            .with_max_level(Level::INFO) // set log level
            .init();

        info!("App started"); // simple log
        let user = "Alice";
        let age = 30;

        info!(%user, %age, "User logged in"); // structured log
    }

    pub fn simple_logs_2()
    {
        tracing_subscriber::fmt().init();

        info!("App started");
        debug!("Debugging details: x = {}", 42);
        warn!("Low disk space");
        error!("Failed to connect to database");
    }

    pub fn simple_logs_3()
    {
        tracing_subscriber::fmt().init();

        let user_id = 101;
        let ip = "192.168.1.15";

        info!(user_id, ip, action = "login", "User login attempt");
    }
}

mod tracing_with_span
{
    use tracing::{info, debug, warn, error, span, Level, Span};
    use tracing_subscriber::fmt::format::FmtSpan;

    pub fn example_1()
    {
        tracing_subscriber::fmt().init();

        let span = span!(Level::INFO, "request", id = 42);
        let _enter = span.enter();

        info!("Processing request");
        info!(step = "db_query", "Fetching user data");
    }

    pub fn example_2()
    {
        tracing_subscriber::fmt().init();

        let span = span!(Level::INFO, "request", id = 42);
        let _enter = span.enter(); // logs inside this block belong to the span

        info!("Processing started");
        info!(step = "db", "Fetching from database");
        info!(step = "cache", "Checking cache");
    }

    pub fn logs_json()
    {
        tracing_subscriber::fmt()
            .json() // log as JSON
            .with_span_events(FmtSpan::CLOSE) // log when spans close
            .init();

        info!(event = "startup", message = "Application started");
        // {"timestamp":"2025-09-05T18:51:09.677715Z","level":"INFO","fields":
        //          {"event":"startup","message":"Application started"},"target":"Logging::tracing"}
    }

    fn do_work()
    {
        let span: Span = span!(Level::DEBUG, "db_query");
        let _enter = span.enter();
        info!("Querying database...");
    }

    pub fn span_with_enter()
    {
        tracing_subscriber::fmt().init();

        let request_span = span!(Level::INFO, "request", id = 42, method = "GET");
        let _enter = request_span.enter();

        info!("Start processing");
        do_work();
        info!("Request finished");
    }
}


mod instrument_tracing
{
    use tracing::{info, debug, warn, error, span, Level, instrument};
    use tracing_subscriber::fmt::format::FmtSpan;

    #[instrument]
    fn divide(a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            error!(a, b, "Division by zero!");
            None
        } else {
            Some(a / b)
        }
    }

    #[instrument]
    fn compute(x: i32, y: i32) -> i32 {
        info!("Starting computation");
        x + y
    }

    #[instrument]
    fn login_unsafe(user: &str, password: &str) -> bool {
        info!("Checking user credentials");
        user == "admin" && password == "12345"
    }

    #[instrument(skip(password))]
    fn login(user: &str, password: &str) -> bool {
        info!("Checking user credentials");
        user == "admin" && password == "12345"
    }

    pub fn instrumental_logs()
    {
        tracing_subscriber::fmt().init();
        divide(10, 0);
    }

    pub fn instrumental_logs_2()
    {
        tracing_subscriber::fmt().init();
        let result = compute(5, 7);
        info!(result, "Computation finished");
    }

    pub fn instrumental_skip_params() {
        tracing_subscriber::fmt().init();
        login("admin", "12345");
        login_unsafe("admin", "12345");
    }
}

mod write_to_file
{
    use tracing::{info, warn};
    use tracing_appender::rolling::RollingFileAppender;
    use tracing_subscriber::{fmt, EnvFilter};

    pub fn append_to_file()
    {
        let file_appender: RollingFileAppender = tracing_appender::rolling::daily("Logging/data/logs", "app.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        tracing_subscriber::fmt()
            .with_writer(non_blocking)
            .with_env_filter(EnvFilter::new("info"))
            .init();

        info!("Service started");
        warn!("Something suspicious happened");
    }
}

mod diff_levels_for_diff_modules
{
    use tracing::info;
    use tracing_subscriber::EnvFilter;

    mod db
    {
        use tracing::debug;
        pub fn connect() {
            debug!("Connecting to DB...");
        }
    }

    mod api
    {
        use tracing::info;
        pub fn start() {
            info!("API server started");
        }
    }

    pub fn demo()
    {
        let filter: EnvFilter = EnvFilter::new("db=debug,api=info");
        tracing_subscriber::fmt()
            // .with_env_filter(filter)
            .init();

        db::connect();
        api::start();
        info!("Main app logic running");
    }

}

pub fn test_all()
{
    tracing_basic::simple_logs();
    // tracing_basic::simple_logs_2();
    // tracing_basic::simple_logs_3();

    // tracing_with_span::example_1();
    // tracing_with_span::example_2();
    // tracing_with_span::logs_json();
    // tracing_with_span::span_with_enter();

    // instrument_tracing::instrumental_logs();
    // instrument_tracing::instrumental_logs_2();
    // instrument_tracing::instrumental_skip_params();

    // write_to_file::append_to_file();

    // diff_levels_for_diff_modules::demo()
}