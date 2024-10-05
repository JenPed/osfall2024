fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main () {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0{
            println!("Fizz");
        } else if num % 5 == 0{
            println!("Buzz");
        } else if is_even(*num){
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }
    }

    let mut sum = 0;
    let mut i = 0;

    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    let mut largest = numbers [0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}