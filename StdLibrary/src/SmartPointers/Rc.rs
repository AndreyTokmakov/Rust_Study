use std::rc::Rc;

fn pointer_to_vector_clone()
{
    let values: Rc<Vec<i32>> = Rc::new(vec![1, 2, 3]);
    
    // The two syntaxes below are equivalent.
    let mut a: Rc<Vec<i32>> = values.clone();
    let mut b: Rc<Vec<i32>> = Rc::clone(&values);

    println!("values: {:?}, a: {:?}, b: {:?}", values, a, b);

    Rc::make_mut(&mut a).push(4);
    println!("values: {:?}, a: {:?}, b: {:?}", values, a, b);
    
    Rc::make_mut(&mut b).push(5);
    println!("values: {:?}, a: {:?}, b: {:?}", values, a, b);
    
    // values: [1, 2, 3], a: [1, 2, 3],    b: [1, 2, 3]
    // values: [1, 2, 3], a: [1, 2, 3, 4], b: [1, 2, 3]
    // values: [1, 2, 3], a: [1, 2, 3, 4], b: [1, 2, 3, 5]
}

fn pointer_to_vector_modify()
{
    let mut values: Rc<Vec<i32>> = Rc::new(vec![1, 2, 3]);

    println!("values: {:?}", values);
    Rc::make_mut(&mut values).push(4);
    println!("values: {:?}", values);
}

fn simple_example()
{
    let a = Rc::new(String::from("heyyy"));
    let b = Rc::clone(&a); // increases the reference counter
    let c = Rc::clone(&a);

    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("Счётчик: {}", Rc::strong_count(&a));
}

fn simple_example_1()
{
    // Создаём данные с подсчётом ссылок
    let a: Rc<i32> = Rc::new(42);
    println!("a = {}, count = {}", a, Rc::strong_count(&a));

    // Клонируем — не копируем данные, только увеличиваем счётчик
    let b: Rc<i32> = Rc::clone(&a);
    println!("b = {}, count = {}", b, Rc::strong_count(&a));

    let c: Rc<i32> = Rc::clone(&a);
    println!("c = {}, count = {}", c, Rc::strong_count(&a));

    // При выходе из области видимости счётчик уменьшается
    drop(b);
    println!("After drop b, count = {}", Rc::strong_count(&a));

    // a = 42, count = 1
    // b = 42, count = 2
    // c = 42, count = 3
    // After drop b, count = 2
}

fn simple_example_2()
{
    let data = Rc::new(String::from("hello"));

    let a = Rc::clone(&data);
    let b = Rc::clone(&data);

    println!("{}", a);
    println!("{}", b);
}

fn reference_count_example()
{
    let v: Rc<i32> = Rc::new(42);
    println!("count = {}", Rc::strong_count(&v));     // 1

    let v2: Rc<i32> = Rc::clone(&v);
    println!("count = {}", Rc::strong_count(&v));     // 2

    {
        let v3: Rc<i32> = Rc::clone(&v);
        println!("count = {}", Rc::strong_count(&v)); // 3
    }
    println!("count = {}", Rc::strong_count(&v));     // 2
}


mod caching
{
    use std::rc::Rc;
    use std::collections::HashMap;

    struct Cache {
        data: HashMap<String, Rc<Vec<u8>>>,
    }

    impl Cache
    {
        fn new() -> Self {
            Cache {
                data: HashMap::new(),
            }
        }

        fn get_or_load(&mut self, key: String) -> Rc<Vec<u8>>
        {
            if let Some(data) = self.data.get(&key)
            {
                println!("Cache hit for: {}", key);
                Rc::clone(data)
            }
            else
            {
                println!("Loading data for: {}", key);
                // We are simulating the loading of big data
                let loaded_data = Rc::new(vec![1, 2, 3, 4, 5]);
                self.data.insert(key, Rc::clone(&loaded_data));
                loaded_data
            }
        }
    }

    pub fn demo()
    {
        let mut cache: Cache = Cache::new();
        let data1: Rc<Vec<u8>> = cache.get_or_load("Data_1".to_string());
        let data2: Rc<Vec<u8>> = cache.get_or_load("Data_2".to_string()); // Из кэша
        let data1_2: Rc<Vec<u8>> = cache.get_or_load("Data_1".to_string());

        println!("All share the same data? {:?}", Rc::ptr_eq(&data1, &data2));
        println!("Data1   count: {}", Rc::strong_count(&data1));
        println!("Data2   count: {}", Rc::strong_count(&data2));
        println!("data1_2 count: {}", Rc::strong_count(&data1_2));

        // Loading data for: Data_1
        // Loading data for: Data_2
        // Cache hit for: Data_1
        // All share the same data? false
        // Data1   count: 3
        // Data2   count: 2
        // data1_2 count: 3
    }
}

mod observer
{
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    trait Observer<T> {
        fn update(&self, newState: &T);
    }

    struct Subject<T> {
        observers: Vec<Weak<RefCell<dyn Observer<T>>>>,
        state: T
    }

    #[derive(Clone)]
    struct Logger {
        id: u32,
    }

    impl Observer<String> for Logger {
        fn update(&self, newState: &String) {
            println!("Logger {}: State changed to '{}'", self.id, newState);
        }
    }

    impl Subject<String>
    {
        fn new(initial: String) -> Self {
            Subject {
                observers: Vec::new(),
                state: initial,
            }
        }

        fn attach(&mut self, observer: Rc<RefCell<dyn Observer<String>>>) {
            self.observers.push(Rc::downgrade(&observer));
        }

        fn setState(&mut self, new_state: String) {
            self.state = new_state;
            self.notify();
        }

        fn notify(&self)
        {
            for weak_observer in &self.observers {
                // upgrade возвращает Option<Rc<RefCell<dyn Observer<String>>>>
                if let Some(observer) = weak_observer.upgrade() {
                    // observer имеет тип Rc<RefCell<dyn Observer<String>>>  borrow() возвращает Ref<dyn Observer<String>>
                    observer.borrow().update(&self.state);
                }
            }
        }
    }

    pub fn demo()
    {
        let mut subject = Subject::new("Initial".to_string());

        // Ключевой момент: явное приведение к типу Rc<RefCell<dyn Observer<String>>>
        let logger1: Rc<RefCell<dyn Observer<String>>> = Rc::new(RefCell::new(Logger { id: 1 }));
        let logger2: Rc<RefCell<dyn Observer<String>>> = Rc::new(RefCell::new(Logger { id: 2 }));

        subject.attach(Rc::clone(&logger1));
        subject.attach(Rc::clone(&logger2));

        subject.setState("New State".to_string());
        drop(logger1); // Удаляем первого наблюдателя
        subject.setState("Final State".to_string());

        // Logger 1: State changed to 'New State'
        // Logger 2: State changed to 'New State'
        // Logger 2: State changed to 'Final State'
    }
}


mod shared_configuration_access
{
    use std::rc::Rc;

    struct Config {
        host: String,
        port: u16,
    }

    impl Config {
        fn new(host: &str, port: u16) -> Self {
            Config {
                host: host.to_string(),
                port,
            }
        }
    }

    struct Database {
        config: Rc<Config>,
    }

    struct Server {
        config: Rc<Config>,
    }

    impl Database {
        fn connect(&self) {
            println!("Connecting to {}:{}", self.config.host, self.config.port);
        }
    }

    impl Server {
        fn start(&self) {
            println!("Starting server on {}:{}", self.config.host, self.config.port);
        }
    }

    pub fn demo()
    {
        // Одна конфигурация для всех компонентов
        let config: Rc<Config> = Rc::new(Config::new("localhost", 8080));

        let db: Database = Database { config: Rc::clone(&config) };
        let server: Server = Server { config: Rc::clone(&config) };

        db.connect();
        server.start();
        println!("Reference count: {}", Rc::strong_count(&config)); // 3 (config + db + server)

        // Connecting to localhost:8080
        // Starting server on localhost:8080
        // Reference count: 3
    }
}

/**
    Rc<T> — разделяемое владение (однопоточное)
    Single-threaded reference-counting pointers. ‘Rc’ stands for ‘Reference Counted’.
    https://doc.rust-lang.org/book/ch15-04-rc.html

    - хранит значение на heap
    - считает, сколько владельцев
    - освобождает память, когда счётчик = 0
    - НЕ потокобезопасен

    Rc нужен Когда:
    - один объект нужен нескольким владельцам
    - нельзя выбрать «главного» owner-а
    - всё происходит в одном потоке
**/
pub fn test_all()
{
    // simple_example();
    // simple_example_1();
    // simple_example_2();
    // reference_count_example();
    // pointer_to_vector_clone();
    // pointer_to_vector_modify();

    // caching::demo();
    // observer::demo();

    shared_configuration_access::demo();
}