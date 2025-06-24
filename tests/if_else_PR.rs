pub fn if_else(x: i32 , y: i32) -> i32{
    if x > y {
        1
    } else if x == y {
        0
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_else() {
        assert_eq!(if_else(5, 3), 1);
        assert_eq!(if_else(3, 3), 0);
        assert_eq!(if_else(4, 4), -1);
    }
}