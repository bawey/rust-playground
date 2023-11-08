use std::io;

mod demos;

fn feature_picker() -> u8 {
    let mut buffer: String = String::new();
    println!("Please insert a feature number");
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            match buffer.trim().parse() {
                Ok(number) => number,
                Err(_) => {
                    println!("Failed to parse!");
                    feature_picker()
                }
            }
        },
        Err(_) => {
            println!("Failed to read!");
            feature_picker()
        }
    }
}

fn main() {
    loop {
        match feature_picker() {
            0 => demos::loops::demo_loop_over_elements(),
            1 => demos::loops::demo_loop_over_range(),
            2 => demos::structs::demo_structs_printing(),
            3 => demos::fibonacci::demo_fibonacci_main(),
            _ => {
                println!("Invalid choice, please retry");
            }
        }
    }
}






