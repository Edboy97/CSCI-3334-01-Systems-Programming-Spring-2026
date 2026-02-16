fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main () {
    let nums: [i32; 10] = [4, 9, 10, 11, 13, 15, 17, 20, 19, 2];
    println!("Analyzing the Numbers");

    for nums in nums.iter(){
        if  nums % 3 == 0 && nums % 5 == 0{
            println!("{}: FizzBuzz", nums);
        }
        else if nums % 3 == 0 {
            println!("{}: Fizz", nums);
        }
        else if nums % 5 == 0 {
            println!("{}: Buzz", nums);
        }
        else {
            if is_even(*nums){
                println!("{}: Even", nums);
            }
            else{
                println!("{}: Odd", nums);
            }
        }
    }
    let mut counter = 0;
    let mut sum = 0;
    while counter < 10{
        sum += nums[counter];
        counter += 1;
    }
    println!("Sum of numbers: {}",sum);
   let mut large = nums[0]; 
   for nums in nums.iter() {
    if *nums > large {
        large = *nums
    }
    }
    println!("Largest Number: {}", large)
}
