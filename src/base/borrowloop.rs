pub fn print_all(nums : &Vec<i32>) {
    for num in nums {
        println!("Number: {}", num); // Print each number in the vector
    }

    println!("{:?}", nums)
}
