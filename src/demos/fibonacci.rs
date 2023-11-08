use std::io;

pub fn demo_fibonacci_main() {
    let mut mem: Vec<u64> = Vec::with_capacity(64);

    mem.push(1);
    mem.push(1);

    'main_loop: loop {
        println!("Which number of the famous Fibonacci sequence would you like to get?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read any input, it seems...");
        let input: usize = input.trim().parse().expect("Hey, that ain't not a number!");
        if input <= 0 {
            println!("This is it, I am tired and in need of a break.");
            break;
        }
        while mem.len() < input {
            let candidate = mem.get(mem.len() - 2).unwrap()
                .saturating_add(*mem.get(mem.len() - 1).unwrap());
            if candidate == u64::MAX {
                println!("Nice try, I can't fly that high! (Try anything lower than {} as input)", input);
                continue 'main_loop;
            }
            mem.push(candidate);
        }
        match mem.get(input - 1) {
            Some(x) => println!("The number is {}", x),
            None => println!("Weird, I should know better!")
        }
    }
}
