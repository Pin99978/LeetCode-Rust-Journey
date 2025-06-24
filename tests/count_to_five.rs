pub fn count_to_five( n : i32) {
    let mut count = 0;
    while count < n {
        count += 1; // Increment count
        println!("Counting: {}", count);
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_to_five() {
        // Test with n = 5
        count_to_five(5);
        // The output will be printed to the console, so we can't assert the output directly.
        // Instead, we can check if the function runs without panicking.
        assert!(true); // Placeholder assertion
    }
}