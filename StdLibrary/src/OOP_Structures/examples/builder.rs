

mod builder
{
    #[derive(Debug)]
    struct User
    {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
        preferences: Option<UserPreferences>,
    }

    #[derive(Debug)]
    struct UserPreferences
    {
        theme: String,
        notifications_enabled: bool,
    }

    #[derive(Default)]
    struct UserBuilder
    {
        username: Option<String>,
        email: Option<String>,
        sign_in_count: u64,
        active: bool,
        preferences: Option<UserPreferences>,
    }

    impl UserBuilder
    {
        fn new() -> Self {
            UserBuilder {
                username: None,
                email: None,
                sign_in_count: 0,
                active: false,
                preferences: None,
            }
        }

        fn username(mut self, username: String) -> Self {
            self.username = Some(username);
            self
        }

        fn email(mut self, email: String) -> Self {
            self.email = Some(email);
            self
        }

        fn sign_in_count(mut self, count: u64) -> Self {
            self.sign_in_count = count;
            self
        }

        fn active(mut self, active: bool) -> Self {
            self.active = active;
            self
        }

        fn preferences(mut self, preferences: UserPreferences) -> Self {
            self.preferences = Some(preferences);
            self
        }

        fn build(self) -> Result<User, &'static str>
        {
            let username: String = self.username.ok_or("Username is required")?;
            let email: String = self.email.ok_or("Email is required")?;

            Ok(User {
                username,
                email,
                sign_in_count: self.sign_in_count,
                active: self.active,
                preferences: self.preferences,
            })
        }
    }

    pub fn demo()
    {
        let user: Result<User, &str> = UserBuilder::new()
            .username(String::from("JohnDoe"))
            .email(String::from("john@example.com"))
            .active(true)
            .build();

        match user {
            Ok(user) => println!("User created successfully: {:?}", user),
            Err(e) => println!("Error creating user: {}", e),
        }
    }
}

pub fn test_all()
{
    builder::demo();
}