use std::io;

fn add(num1: &f64, num2: &f64) -> f64 {

    num1 + num2
}

fn sub(num1: &f64, num2: &f64) -> f64 {

    num1 - num2
}

fn mul(num1: &f64, num2: &f64) -> f64 {

    num1 * num2
}

fn div(num1: &f64, num2: &f64) -> f64 {

    if num2 > &0.0 {

        num1/ num2

    } else {

        println!(" The denominator cannot be zero");
        0.0

    }
}

fn pow(num1: &f64, num2: &f64) -> f64 {

    num1**num2
}

fn speed(distance: &f64, time: &f64) -> f64{

    div(distance, time)
}

fn bmi(weight: &f64, height: &f64) -> f64{

    let power: f64 = 2.0;
    div(weight, &pow(height,&power))

}

fn main(){

    println!(" Hi this is a basic calculator.");
    println!(" This calculators will ask you to enter two numbers");
    println!(" This calculator let's you perform the following actions: \n ");
    println!("1. Addition : Number 1 gets added to Number 2 \n");
    println!("2. Subtraction : Number 2 gets deducted from Number 1 \n");
    println!("3. Multiplication : Number 1 gets multiplied by Number 2\n");
    println!("4. Division : Number 1 gets divided by Number 2 \n");
    println!("5. Power : Number 1 is raised to the power of Number 2 \n");
    println!("6. Speed : Number 1 is the distance covered and  Number 2 is the time taken \n");
    println!("7. BMI : Number 1 is the person's weight in Kilograms and Number 2 is person's height in meters\n");

    let mut operation = String::new();
    println!("Please enter the operation number (1-7): ");
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read input.");
    let operation: u32 = operation.trim().parse().expect("Please enter a valid number.");


    // Take input for number 1
    let mut num1 = String::new();
    println!("Please enter number 1: ");
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read input.");
    let num1: f64 = num1.trim().parse().expect("Please enter a valid number.");

    // Take input for number 2
    let mut num2 = String::new();
    println!("Please enter number 2: ");
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read input.");
    let num2: f64 = num2.trim().parse().expect("Please enter a valid number.");

    match operation {
        1 => println!("Result: {}", add(&num1, &num2)),
        2 => println!("Result: {}", sub(&num1, &num2)),
        3 => println!("Result: {}", mul(&num1, &num2)),
        4 => println!("Result: {}", div(&num1, &num2)),
        5 => println!("Result: {}", pow(&num1, &num2)),
        6 => println!("Speed: {}", speed(&num1, &num2)),
        7 => println!("BMI: {}", bmi(&num1, &num2)),
        _ => println!("Invalid operation."),
    }


}