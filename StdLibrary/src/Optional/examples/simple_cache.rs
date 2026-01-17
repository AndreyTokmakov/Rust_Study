
mod caching
{
    struct Cache {
        data: Option<String>,
    }

    impl Cache {
        fn new() -> Self {
            Cache { data: None }
        }

        fn get_data(&self) -> Option<&String> {
            self.data.as_ref()
        }

        fn set_data(&mut self, value: String) {
            self.data = Some(value);
        }

        fn clear(&mut self) {
            self.data = None;
        }
    }

    pub fn demo()
    {
        let mut cache: Cache = Cache::new();

        // Initially the cache is empty
        match cache.get_data() {
            Some(data) => println!("Cache hit: {}", data),
            None => println!("Cache miss"),
        }
        // Output: Cache miss

        // Set some data
        cache.set_data("Important information".to_string());

        // Now we get a cache hit
        if let Some(data) = cache.get_data() {
            println!("Cache hit: {}", data);
        }
        // Clear the cache
        cache.clear();

        // Use unwrap_or to provide a default when cache is empty
        let default_value: String = "DEFAULT".to_string();
        let data: &String = cache.get_data().unwrap_or(&default_value);
        println!("Data: {}", data);

        // Cache miss
        // Cache hit: Important information
        // Data: DEFAULT
    }
}

// INFO: https://www.compilenrun.com/docs/language/rust/rust-enums/rust-option-enum#example-3-implementing-a-simple-cache
pub fn test_all()
{
    caching::demo();
}