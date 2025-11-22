fn setup() -> Vec<i32> {
    vec![1, 2, 3]
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_len()
    {
        let data: Vec<i32> = setup();
        assert_eq!(data.len(), 3);
    }

    #[test]
    fn test_first()
    {
        let data: Vec<i32> = setup();
        assert_eq!(data[0], 1);
    }
}
