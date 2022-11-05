use std::io;

fn main() {
    'main_loop: loop {
        println!("\n\nPlease input your types of degrees OR type X to exit:\n\n");

        let mut d_type = String::new();
        io::stdin()
            .read_line(&mut d_type)
            .expect("failed to read line");

        loop {
            match d_type.trim() {
                "F" => println!("\n\nPlease input your Celsius degrees\n"),
                "f" => println!("\n\nPlease input your Celsius degrees\n"),
                "C" => println!("\n\nPlease input your Fahrenheit degrees\n"),
                "c" => println!("\n\nPlease input your Fahrenheit degrees\n"),
                "X" => break 'main_loop,
                "x" => break 'main_loop,
                &_ => {
                    print!("{}", d_type);
                    println!("\n\nChoose your the type of degrees \n into which you would like to convert your value ! (C or F)\n\n\n\n\n");
                    break;
                }
            }

            let mut degree = String::new();
            io::stdin()
                .read_line(&mut degree)
                .expect("failed to read line");

            let parsed_degree: i16 = match degree.trim().parse() {
                Ok(num) => num,
                Err(error) => {
                    println!("Error is : {} \nPlease enter a number !", error.to_string());
                    continue;
                }
            };

            match d_type.trim() {
                "F" => {
                    println!(
                        "\n\n{} C is {} F\n\n\n",
                        parsed_degree,
                        convert_to_fahrenheit(parsed_degree)
                    );
                    break;
                }
                "f" => {
                    println!(
                        "\n\n{} C is {} F\n\n\n\n",
                        parsed_degree,
                        convert_to_fahrenheit(parsed_degree)
                    );
                    break;
                }
                "C" => {
                    println!(
                        "\n\n{} F is {} C\n\n\n\n\n",
                        parsed_degree,
                        convert_to_celsius(parsed_degree)
                    );
                    break;
                }
                "c" => {
                    println!(
                        "\n\n{} F is {} C\n\n\n\n",
                        parsed_degree,
                        convert_to_celsius(parsed_degree)
                    );
                    break;
                }
                &_ => continue,
            }
        }
    }
}

fn convert_to_fahrenheit(degrees: i16) -> i16 {
    (degrees * 9 / 5) + 32
}
fn convert_to_celsius(degrees: i16) -> i16 {
    (degrees - 32) * 5 / 9
}
