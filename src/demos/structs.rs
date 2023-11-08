#[derive(Debug)]
pub struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn new(last_name: &str, first_name: &str, age: u8) -> Person {
        let last_name = String::from(last_name);
        let first_name = String::from(first_name);
        Person { first_name, last_name, age }
    }
}


pub fn demo_structs_printing() {
    let p1 = Person::new("Dow", "Jones", 138);
    let p2: &Person = &Person::new("Doe", "Jane", 35);

    println!("One person: {:?}", p1);
    println!("Two person: {:?}", p2);


    let option_1 = Some(p1);
    // reference due to some binding magic that "pushes the ref down" w/o ownership transfers
    match &option_1 {
        None => println!("there be nothing"),
        Some(p) => println!("Person found: {:?}", p)
    }

    if let Some(p) = &option_1 {
        println!("The following person was discovered with if-let: {:?}", p);
    }

    println!("Person one again: {:?}", option_1);
}