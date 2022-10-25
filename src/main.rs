use rand::{self, Rng};
use std::num::ParseIntError;
use std::io;
use std::io::Write;

fn get_num_input() -> Result<i32, String>{
    let mut raw_input = String::new();
    std::io::stdin().read_line(&mut raw_input).unwrap();
        
    let result_of_parse: Result<i32, ParseIntError> = raw_input.as_str().trim().parse();

    return match result_of_parse{
        Ok(res) => Ok(res),
        Err(_) => Err(raw_input),
    }
}

fn get_num_input_complete() -> i32{
    loop{
        print!("> ");
        io::stdout().flush().unwrap();
        let input_value = get_num_input();

        match input_value{
            Ok(val) => return val,
            Err(input) => print!("Could not recognize input {}", input),
        }
    }
}
fn main() {
    println!("Welcome to Henry Tischlers Number Memorization Extravaganza!!!!");
    println!();

    println!("Please select one of the following sequences to study: ");
    println!("1. Square Numbers");
    println!("2. Cube Numbers");

    let exponent = get_num_input_complete() + 1;

    println!("Please select one of the following ways to study these numbers: ");
    println!("1. Values only (ex. 3^2 = ?)");
    println!("2. Roots only (ex. sqrt of 14 = ?)");
    println!("3. both");
    let option = get_num_input_complete();

    let mut value_probability = 0.0;
    let mut roots_probability = 0.0;

    
    if option == 1{
        value_probability = 1.0;
    }
    else if option == 2{
        roots_probability = 1.0;
    }
    else if option == 3{
        value_probability = 0.5;
        roots_probability = 0.5;
    }
    
    let mut rng = rand::thread_rng();

    loop{
        let root_val: f64 = roots_probability * rng.gen::<f64>();
        let value_val: f64 = value_probability * rng.gen::<f64>();

        let value_before_exp: i32= rng.gen_range(0..=25);

        if root_val > value_val{
            println!("Find 	√{}", value_before_exp.pow(exponent as u32).to_string());

            let input = get_num_input_complete();

            if input == value_before_exp{
                println!("CORRECT!!!");
            }
            else{
                println!("Incorrect, the answer is actually {}", value_before_exp);
            }
        }
        else{
            println!("Find {}^{}", value_before_exp, exponent.to_string());
                        
            let input = get_num_input_complete();

            if input == value_before_exp.pow(exponent as u32){
                println!("CORRECT!!!!");
            }
            else{
                println!("INCORRECT, the answer is actually {}", value_before_exp.pow(2));
            }
        }

        println!("");
    }
}