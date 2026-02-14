
mod example_one
{
    #[derive(Debug)]
    struct Config
    {
        host: String,
        port: u16,
        timeout_ms: u64,
        use_tls: bool,
    }

    struct ConfigBuilder
    {
        host: String,
        port: u16,
        timeout_ms: u64,
        use_tls: bool,
    }

    impl ConfigBuilder
    {
        fn new() -> Self
        {
            Self
            {
                host: "127.0.0.1".into(),
                port: 8080,
                timeout_ms: 1000,
                use_tls: false,
            }
        }

        fn host(mut self, host: &str) -> Self {
            self.host = host.into();
            self
        }

        fn port(mut self, port: u16) -> Self {
            self.port = port;
            self
        }

        fn timeout(mut self, timeout_ms: u64) -> Self {
            self.timeout_ms = timeout_ms;
            self
        }

        fn tls(mut self, enabled: bool) -> Self {
            self.use_tls = enabled;
            self
        }

        fn build(self) -> Config
        {
            Config {
                host: self.host,
                port: self.port,
                timeout_ms: self.timeout_ms,
                use_tls: self.use_tls,
            }
        }
    }

    fn startServer(config: Config)
    {
        println!("Starting server:");
        println!("Host: {}", config.host);
        println!("Port: {}", config.port);
        println!("Timeout: {} ms", config.timeout_ms);
        println!("TLS: {}", config.use_tls);
    }

    pub fn demo()
    {
        let config = ConfigBuilder::new()
            .host("0.0.0.0")
            .port(9000)
            .timeout(5000)
            .tls(true)
            .build();

        startServer(config);
    }

    // Starting server:
    // Host: 0.0.0.0
    // Port: 9000
    // Timeout: 5000 ms
    // TLS: true
}

pub fn test_all()
{
    example_one::demo();
}