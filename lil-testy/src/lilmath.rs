pub fn add(x: i64, y:i64) -> i64{
    x+y
}

pub fn mult(x: i64, y:i64) -> i64{
    x*y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2,2), 4);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(add(2, -2), 0);
    }

    #[test]
    fn test_add_negative(){
        assert_eq!(add(-2, -2), -4);
    }
}
