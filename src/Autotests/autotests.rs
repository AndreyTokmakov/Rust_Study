
fn devide(a: i32, b: i32) -> i32 {
    if 0 == b {
        panic!("PANIC: 0 devide!");
    }
    a / b
}

#[test]
fn check_equals() {
    let result = devide(8, 2);
    assert_eq!(result, 4);
}

#[test]
fn check_not_equals() {
    let result = 2 + 2;
    assert_ne!(result, 5);
}

#[test]
fn exploration() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn should_fail() {
    panic!("Make this test fail");
}

#[test]
#[should_panic(expected = "PANIC: 0 devide!")]
fn should_panic_test() {
    let result = devide(8, 0);
    assert_eq!(result, 4);
}

#[test]
#[ignore]
fn test_will_be_IGNORED() {
    // code that takes an hour to run
}

//--------------------------------------------------------------------

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests_modules_1 {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let larger = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!larger.can_hold(&larger));
    }
}


pub fn test_all()
{
    // TODO: Run
    //      cargo test
    //      cargo test tests_modules_1
    //      cargo test -- --test-threads=1

    // TODO: To filter by name:
    //      cargo test equals    <--- This runs test which name contains 'equals'
}