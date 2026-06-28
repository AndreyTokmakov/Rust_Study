
pub mod helpers
{
    use std::fmt;
    use std::fmt::Display;

    #[derive(Debug, Clone)]
    pub struct Tracked<T> {
        value: T,
    }

    impl<T: std::fmt::Debug> Tracked<T>
    {
        pub fn new(value: T) -> Self {
            let obj = Tracked { value };
            println!(" => {:?} created", &obj);
            obj
        }
    }

    impl<T> std::ops::Deref for Tracked<T>
    {
        type Target = T;

        fn deref(&self) -> &T {
            &self.value
        }
    }

    impl<T: Display> Display for Tracked<T>
    {
        fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
            write!(format, "Tracked({})", &self.value)
        }
    }
}

mod simple_example
{
    use std::cell::{Ref, RefCell, RefMut};
    use std::collections::HashMap;
    use std::hash::Hash;
    use crate::cache::helpers::Tracked;

    struct Cache<K, V>  {
        data: RefCell<HashMap<K, V>>,
    }

    impl<K, V> Cache<K, V>
        where K: Eq + Hash + Clone,
              V: Clone
    {
        fn new() -> Self {
            Cache {
                data: RefCell::new(HashMap::new()),
            }
        }

        fn get(&self, key: &K) -> Option<V> {
            self.data.borrow().get(key).cloned()
        }

        fn get_or_insert(&self, key: K, default: V) -> V {
            self.data.borrow_mut().entry(key.clone()).or_insert_with(|| default).clone()
        }

        fn set(&self, key: K, value: V) {
            self.data.borrow_mut().insert(key, value);
        }
    }

    pub fn demo()
    {
        let cache: Cache<String, Tracked<String>> = Cache::new();

        cache.set("name".to_owned(), Tracked::new("Rust".to_owned()));
        cache.set("type".to_owned(),  Tracked::new("Language".to_owned()));

        println!("Name: {:?}", cache.get(&"name".to_string()));
        println!("Type: {:?}", cache.get(&"type".to_string()));

        let lang: Tracked<String> = cache.get_or_insert("type".to_owned(), Tracked::new("Unknown".to_owned()));
        println!("Type: {}", lang);  // "Language"

        let version: Tracked<String> = cache.get_or_insert("version".to_owned(), Tracked::new("1.0".to_owned()));
        println!("Version: {}", version);  // "1.0"
    }
}

pub fn test_all()
{
    simple_example::demo();

    //  => Tracked { value: "Rust" } created
    //  => Tracked { value: "Language" } created
    // Name: Some(Tracked { value: "Rust" })
    // Type: Some(Tracked { value: "Language" })
    //  => Tracked { value: "Unknown" } created
    // Type: Tracked(Language)
    //  => Tracked { value: "1.0" } created
    // Version: Tracked(1.0)
}
