use std::io;

fn main() {

    println!("Simple Rust(y) Calculator");

    loop {
        println!("\nChoose an Operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        //trim() removes whitespaces.
        //parse will convert the input into the specified type which is u32 in this case.
        //match is used to handle any error should they arise.
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 5.");
                continue;
            }
        };

        if choice == 5 {
            println!("Exiting this rusty place...");
            break;
        }

        println!("Enter the first number: ");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Failed to read first number.");
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number.");
                continue
            }
        };


        println!("Enter the second number: ");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read second number.");
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number.");
                continue
            }
        };


        match choice {

            1 => println!("Result: {} + {} = {}", num1, num2, num1 + num2),
            2 => println!("Result: {} - {} = {}", num1, num2, num1-num2),
            3 => println!("Result: {} x {} = {}", num1, num2, num1*num2),
            4 => {
                if num2 == 0.0 {
                    println!("Error, division by zero.");
                } else {
                    println!("Result: {} / {} = {}", num1, num2, num1 / num2);
                }
            }
            _ => println!("Invalid choice, please select a valid choice"),
        }



    }

}

