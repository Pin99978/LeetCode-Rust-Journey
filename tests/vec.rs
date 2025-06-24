pub fn sum_vec(nums: Vec<i32>) -> i32{
    let mut sum = 0;
    for num in nums {
        sum += num; // Add each number to the sum
    }
    sum
}
// rewrite with iter
pub fn sum_vec_iter(nums: Vec<i32>) -> i32 {
    nums.iter().sum()
}

// Rewrite using recursion 

pub fn sum_vec_recursive(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        0 // Base case: if the slice is empty, return 0
    } else {
        nums[0] + sum_vec_recursive(&nums[1..]) // Recursive case: add the first element to the sum of the rest
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_sum_vec(){
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_vec(nums), 15); // Test the sum of the vector
    }

    #[test]
    fn test_sum_vec_iter() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_vec_iter(nums), 15); // Test the sum using iterator
    }

    #[test]
    fn test_sum_vec_recursive() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_vec_recursive(&nums), 15); // Test the sum using recursion
    }
}

