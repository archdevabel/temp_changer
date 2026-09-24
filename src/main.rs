use std::io;

fn main() {

    loop {
    let mut temperature = String::new();
    let mut choice = String::new();
    let mut close = String::new();

        println!("Please choose Fahrenheit (1) or Celsius (2)");

        io::stdin()
            .read_line(&mut choice)
            .expect("Fail to read choice");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid number 1 or 2");
                return;
            }
        };

        println!("Please put the temperature:");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Fail to read input");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid number!");
                return;
            }
        };

        if choice == 1 {
            let converted = (temperature - 32.0) * 5.0 / 9.0;
            println!("It's {:.2} Celsius!", converted);
        } else if choice == 2 {
            let converted = (temperature *  5.0 / 9.0) + 32.0;
            println!("It's {:.2} Fahrenheit", converted);
        } else {
            println!("Invalid choice pleae put 1 or 2");
        }

        println!("Want to quit? y/n");
        io::stdin()
            .read_line(&mut close)
            .expect("Fail to read input");

        let close = close.trim();

        if close == "y" {
            println!("Goodbye!");
            break;
        }
    }

}
