
mod newtype_from_int_1
{
    #[derive(Debug)]
    struct Years(i64);

    pub fn demo()
    {
        // Using the tuple or destructuring syntax to obtain the new_type's value as the base type

        let years: Years = Years(42);
        let years_as_primitive_1: i64 = years.0;      // Tuple
        let Years(years_as_primitive_2) = years; // Destructuring

        println!("years: {:?}", years);
        println!("years_as_primitive_1: {:?}", years_as_primitive_1);
        println!("years_as_primitive_2: {:?}", years_as_primitive_2);
    }
}

mod newtype_from_int_2
{
    struct Years(i64);
    struct Days(i64);

    impl Years
    {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }

    impl Days
    {
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }

    fn is_adult(age: &Years) -> bool {
        age.0 >= 18
    }

    pub fn demo()
    {
        let age: Years = Years(25);
        let age_days: Days = age.to_days();

        println!("Is an adult? {}", is_adult(&age));
        println!("Is an adult? {}", is_adult(&age_days.to_years()));
        // println!("Is an adult? {}", is_adult(&age_days));
    }
}


mod newtype_from_string
{
    struct Hostname(String);

    fn connect(host: Hostname) {
        println! ("connected to {}", host.0);
    }

    pub fn demo()
    {
        let ordinary_string: String = String::from("0.0.0.0");
        let host: Hostname = Hostname ( ordinary_string.clone() );

        // connect(ordinary_string); // INFO: Will not compile
        connect(host);
    }
}

pub fn test_all()
{
    newtype_from_int_1::demo();
    // newtype_from_int_2::demo();
    // newtype_from_string::demo();
}