fn find_first_even(nums: Vec<i32>) -> Option<i32> {
    let mut i = 0;
    loop {
        if i >= nums.len(){
            return None;
        }
        if nums[i] % 2 == 0 {
            return Some(nums[i]); // Return the first even number found
        }
        i += 1;
    }


}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_find_first_even() {
        let nums = vec![1, 3, 5, 6, 7, 8];
        assert_eq!(find_first_even(nums), Some(6)); // Test with a vector containing even numbers
    }
}
