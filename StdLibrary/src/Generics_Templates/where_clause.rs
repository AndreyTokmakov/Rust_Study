mod basic_example
{
    use std::fmt;
    use std::fmt::{Debug, Display};

    #[derive(Debug)]
    struct Request
    {
        data: String
    }

    #[derive(PartialEq)]
    struct Response
    {
        data: String
    }

    impl Display for Response
    {
        fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
            write!(format, "Response [ data = {}]",self.data)
        }
    }

    fn process_data_1<T: Clone + Debug,U: Display + PartialEq>(data: T, output: U)
    {
        println!("Process data: {:?} ==> Output: {}", data, output);
    }

    fn process_data<T, U>(data: T, output: U)
        where T: Clone + Debug,
              U: Display + PartialEq,
    {
        println!("Process data: {:?} ==> Output: {}", data, output);
    }

    pub fn demo()
    {
        let request: Request = Request { data: "Payload".to_string() };
        let response: Response = Response { data: "OutPut".to_string() };

        process_data_1(&request, &response);
        process_data(&request, &response);
    }
}

pub fn test_all()
{
    basic_example::demo();
}