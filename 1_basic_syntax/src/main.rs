fn main() {
    println!("Hello, world!");

    for_sample();
    if_sample();
}

fn for_sample() {
    for x in 0..5 {
        println!("{}", x)
    }
}

fn if_sample() {
    let x = 10;
    if x < 8 {
        println!("yes");
    } else {
        println!("no");
    }
}
