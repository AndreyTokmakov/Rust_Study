

mod basic
{
    struct Gzip;
    struct Lz4;

    trait CompressionStrategy {
        fn compress(&self, data: &str) -> String;
    }

    struct Compressor {
        strategy: Box<dyn CompressionStrategy>,
    }

    impl CompressionStrategy for Gzip
    {
        fn compress(&self, data: &str) -> String {
            format!("GZIP({})", data)
        }
    }

    impl CompressionStrategy for Lz4
    {
        fn compress(&self, data: &str) -> String {
            format!("LZ4({})", data)
        }
    }

    impl Compressor
    {
        fn new(strategy: Box<dyn CompressionStrategy>) -> Self {
            Self { strategy }
        }

        fn compress(&self, data: &str) -> String {
            self.strategy.compress(data)
        }

        fn set_strategy(&mut self, strategy: Box<dyn CompressionStrategy>) {
            self.strategy = strategy;
        }
    }

    pub fn demo()
    {
        let mut compressor: Compressor = Compressor::new(Box::new(Gzip));
        println!("{}", compressor.compress("data"));

        compressor.set_strategy(Box::new(Lz4));
        println!("{}", compressor.compress("data"));
    }

    // GZIP(data)
    // LZ4(data)
}

mod no_heap_example
{
    struct Gzip;
    struct Lz4;


    impl CompressionStrategy for Gzip
    {
        fn compress(&self, data: &str) -> String {
            format!("GZIP({})", data)
        }
    }

    impl CompressionStrategy for Lz4
    {
        fn compress(&self, data: &str) -> String {
            format!("LZ4({})", data)
        }
    }

    trait CompressionStrategy {
        fn compress(&self, data: &str) -> String;
    }

    struct Compressor<S: CompressionStrategy> {
        strategy: S,
    }

    impl<S: CompressionStrategy> Compressor<S>
    {
        fn new(strategy: S) -> Self {
            Self { strategy }
        }

        fn compress(&self, data: &str) -> String {
            self.strategy.compress(data)
        }
    }

    pub fn demo()
    {
        let compressor = Compressor::new(Gzip);
        println!("{}", compressor.compress("data"));

        let compressor = Compressor::new(Lz4);
        println!("{}", compressor.compress("data"));
    }

    // GZIP(data)
    // LZ4(data)
}

pub fn test_all()
{
    // basic::demo();
    no_heap_example::demo();
}