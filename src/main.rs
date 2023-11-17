use std::io;

fn main() {
    loop {
        println!("Type a number 1 for converting F to C and 2 for converting C to F");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input == 1 {
            let mut input = String::new();
            println!("Type a temperature in fahrenheit");
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            let input: f64 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let output = calculate_celsius(input);
            println!("{input} fahrenheit in celsius is: {output}");
            break;
        } else if input == 2 {
            let mut input = String::new();
            println!("Type a temperature in celsius");

            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            let input: f64 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let output = calculate_fahrenheit(input);
            println!("{input} celsius in fahrenheit is: {output}");
            break;
        } else {
            println!("Try again and type a valid number!!");
        }
    }
}

fn calculate_fahrenheit(x: f64) -> f64 {
    x * (9.0 / 5.0) + 32.0
}

fn calculate_celsius(x: f64) -> f64 {
    5.0 / 9.0 * (x - 32.0)
}
