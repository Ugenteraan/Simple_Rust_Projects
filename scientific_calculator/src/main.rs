use std::io;


//helper func to read number.
fn read_number(prompt: &str) -> f64 {
    
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read the line.");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number. Please try again!"),
        }
    }
}

fn main() {

    println!("Rustific Calculator");

    loop {
        println!("\nChoose an Operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Sqrt");
        println!("6. Pow");
        println!("7. Sin");
        println!("8. Cos");
        println!("9. Tan");
        println!("10. Exit");

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

        if choice == 10 {
            println!("Exiting this rusty place...");
            break;
        }

        if choice >= 1 && choice <= 4 {

            let num1 = read_number("Enter the first number: ");
            let num2 = read_number("Enter the second number: ");

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

        } else if choice == 5 {
            
            let num = read_number("Enter a number: ");
            if num < 0.0 {
                println!("Error: Cannot calculate the square root of a negative number.");
            } else {
                println!("Result: sqrt({}) = {}", num, num.sqrt());
            }
        } else if choice == 6 {
            
            let base = read_number("Enter the base number: ");
            let exponent = read_number("Enter the exponent number: ");

            println!("Result: {}^{} = {}", base, exponent, base.powf(exponent));
        } else if choice >= 7 && choice <= 9 {
            
            let angle = read_number("Enter the angle in degrees: ");
            let radians = angle.to_radians(); //convert to radian for calculation.
           
            match choice {
                7 => println!("Result: sin({}) = {}", angle, radians.sin()),
                8 => println!("Result: cos({}) = {}", angle, radians.cos()),
                9 => if angle == 90.0 {
                    println!("Tan(90) yields sin(90)/cos(90) which is 1/0. Thus, it's undefined.");
                } else{
                    println!("Result: tan({}) = {}", angle, radians.tan());
                },
                _ => {}
            }
        } else{
            println!("Invalid choice! Please select a valid operation.");
        }
    }
}

