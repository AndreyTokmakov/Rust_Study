
pub mod user_repository
{
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    struct User {
        id: u64,
        name: String,
    }

    trait UserRepository
    {
        fn get_by_id(&self, id: u64) -> Option<User>;
        fn save(&mut self, user: User);
        fn delete(&mut self, id: u64);
    }

    struct InMemoryUserRepository {
        users: HashMap<u64, User>,
    }

    impl InMemoryUserRepository
    {
        fn new() -> Self {
            Self {
                users: HashMap::new(),
            }
        }
    }

    impl UserRepository for InMemoryUserRepository
    {
        fn get_by_id(&self, id: u64) -> Option<User> {
            self.users.get(&id).cloned()
        }

        fn save(&mut self, user: User) {
            self.users.insert(user.id, user);
        }

        fn delete(&mut self, id: u64) {
            self.users.remove(&id);
        }
    }

    struct UserService<R>
        where R: UserRepository
    {
        repository: R
    }

    impl<R> UserService<R>
        where R: UserRepository
    {
        fn new(repository: R) -> Self {
            Self { repository }
        }
    }

    impl<R> UserService<R>
        where R: UserRepository
    {
        fn create_user(&mut self, id: u64, name: String ) {
            let user = User { id, name };
            self.repository.save(user);
        }

        fn find_user(&self, id: u64, ) -> Option<User> {
            self.repository.get_by_id(id)
        }

        fn remove_user(&mut self, id: u64) {
            self.repository.delete(id);
        }
    }

    struct PostgresUserRepository {
        connection_string: String,
    }

    impl UserRepository for PostgresUserRepository
    {
        fn get_by_id(&self, id: u64) -> Option<User> {
            println!("SELECT * FROM users WHERE id={}", id);
            None
        }

        fn save(&mut self, user: User) {
            println!("INSERT user {}", user.id);
        }

        fn delete(&mut self, id: u64) {
            println!("DELETE user {}", id);
        }
    }

    pub fn testPGDatabase()
    {
        let repo: PostgresUserRepository = PostgresUserRepository {
            connection_string: "postgres://...".into()
        };
        let mut service: UserService<PostgresUserRepository> = UserService::new(repo);

        service.create_user(1, "Alice".into());
        service.create_user(2, "Bob".into());

        println!("{:?}", service.find_user(1));

        service.remove_user(1);

        println!("{:?}", service.find_user(1));

        // INSERT user 1
        // INSERT user 2
        // SELECT * FROM users WHERE id=1
        // None
        // DELETE user 1
        // SELECT * FROM users WHERE id=1
        // None
    }

    pub fn demo_InMemoryDatabase()
    {
        let repo: InMemoryUserRepository = InMemoryUserRepository::new();
        let mut service: UserService<InMemoryUserRepository> = UserService::new(repo);

        service.create_user(1, "Alice".into());
        service.create_user(2, "Bob".into());

        println!("{:?}", service.find_user(1));

        service.remove_user(1);

        println!("{:?}", service.find_user(1));

        // Some(User { id: 1, name: "Alice" })
        // None
    }


    pub fn demo()
    {
        testPGDatabase();
        // demo_InMemoryDatabase();
    }
}