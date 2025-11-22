
#[cfg(test)]
mod mock_storage_tests
{
    use mockall::*;

    #[automock]
    pub trait Storage {
        fn save(&self, data: i32) -> bool;
    }

    pub fn process<S: Storage>(s: &S, val: i32) -> bool {
        s.save(val * 2)
    }

    #[test]
    fn test_process()
    {
        let mut mock_storage: MockStorage = MockStorage::new();
        mock_storage.expect_save().withf(|x| *x == 20).return_const(true);

        assert!(process(&mock_storage, 10));
    }
}

#[cfg(test)]
mod calculator_tests
{
    use mockall::*;

    #[automock]
    pub trait Calculator {
        fn add(&self, a: i32, b: i32) -> i32;
        fn mul(&self, a: i32, b: i32) -> i32;
    }

    pub fn compute<T: Calculator>(c: &T, x: i32) -> i32 {
        c.add(x, 1) + c.mul(x, 2)
    }


    #[test]
    fn test_compute()
    {
        let mut calculator_mock: MockCalculator = MockCalculator::new();

        calculator_mock.expect_add().withf(|a, b| *a == 10 && *b == 1).return_const(11);
        calculator_mock.expect_mul().withf(|a, b| *a == 10 && *b == 2).return_const(20);

        assert_eq!(compute(&calculator_mock, 10), 31);
    }
}

#[cfg(test)]
mod result_tests // Мок, возвращающий Result<T, E>
{
    use mockall::*;

    #[automock]
    pub trait Storage {
        fn load(&self, key: &str) -> Result<String, &'static str>;
    }

    pub fn read_value<S: Storage>(s: &S) -> String {
        s.load("name").unwrap_or("default".into())
    }

    #[test]
    fn test_ok()
    {
        let mut mock = MockStorage::new();
        mock.expect_load().returning(|_| Ok("Bob".to_string()));
        assert_eq!(read_value(&mock), "Bob");
    }

    #[test]
    fn test_err()
    {
        let mut mock = MockStorage::new();
        mock.expect_load().returning(|_| Err("not found"));
        assert_eq!(read_value(&mock), "default");
    }
}

#[cfg(test)]
mod checking_the_number_of_calls
{
    use mockall::*;

    #[automock]
    pub trait Logger {
        fn log(&self, msg: &str);
    }

    pub fn do_work<L: Logger>(l: &L) {
        l.log("start");
        l.log("end");
    }

    #[test]
    fn test_call_count()
    {
        let mut logger_mock: MockLogger = MockLogger::new();

        logger_mock.expect_log()
            .times(2)
            .with(predicate::eq("start"))
            .return_const(());

        logger_mock.expect_log()
            .times(2)
            .with(predicate::eq("end"))
            .return_const(());

        do_work(&logger_mock);
        do_work(&logger_mock);
    }
}

#[cfg(test)]
mod moc_complex_structure_arguments
{
    use mockall::*;

    #[derive(Debug, PartialEq)]
    pub struct User {
        pub id: u32,
    }

    #[automock]
    pub trait Repo {
        fn save(&self, u: User) -> bool;
    }

    pub fn save_user<R: Repo>(repo: &R, id: u32) -> bool {
        repo.save(User { id })
    }

    #[test]
    fn test_repo()
    {
        let mut mock_repo: MockRepo = MockRepo::new();

        mock_repo.expect_save().withf(|u: &User| u.id == 55).return_const(true);
        assert!(save_user(&mock_repo, 55));
    }
}

#[cfg(test)]
mod mock_with_call_sequence
{
    use mockall::*;
    use mockall::Sequence;

    #[automock]
    pub trait Handler {
        fn step(&self, n: u32);
    }

    pub fn flow<H: Handler>(h: &H) {
        h.step(1);
        h.step(2);
        h.step(3);
    }

    #[test]
    fn test_sequence()
    {
        let mut handler_mock: MockHandler = MockHandler::new();
        let mut sequence: Sequence = Sequence::new();

        handler_mock.expect_step().times(1).with(predicate::eq(1))
            .in_sequence(&mut sequence).return_const(());
        handler_mock.expect_step().times(1).with(predicate::eq(2))
            .in_sequence(&mut sequence).return_const(());
        handler_mock.expect_step().times(1).with(predicate::eq(3))
            .in_sequence(&mut sequence).return_const(());

        flow(&handler_mock);
    }
}