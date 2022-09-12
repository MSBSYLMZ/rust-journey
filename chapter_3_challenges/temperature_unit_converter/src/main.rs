use std::io;

fn main() {
    loop {
        println!("Please input the unit of the temperature that you are about the input.");
        println!("F for Fahrenheit, C for Celsius");
        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");
        let degree: f64;
        loop {
            println!("Please input the degree in numbers");
            let mut degree_str = String::new();

            io::stdin()
                .read_line(&mut degree_str)
                .expect("Failed to read line");

            match degree_str.trim().parse::<f64>() {
                Ok(i) => {
                    degree = i;
                    break;
                }
                Err(_err) => {
                    continue;
                }
            };
        }
        
        let unit = unit.trim();

        if unit == "F" || unit == "f" {
            let result = (degree - 32.0) / 1.8;
            println!("The {degree} Fahrenheit degree is equal to {result} Celsius Degree");
            break;
        } else if unit == "C" || unit == "c" {
            let result = (degree * 1.8) + 32.0;
            println!("The {degree} Celsius degree is equal to {result} Fahrenheit Degree");
            break;
        }
       
    }
}
