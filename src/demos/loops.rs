pub fn demo_loop_over_elements() {
    let needle = 15;
    println!("The needle is {}", needle);
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    for item in haystack {
        if item == needle {
            println!("{}", item);
        }
    }

    println!("Behold the haystack: {:?}", haystack);
}

pub fn demo_loop_over_range() {
    for i in 0..123 {
        println!("The counter is at {}", i);
    }
}