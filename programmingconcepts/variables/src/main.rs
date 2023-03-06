fn main() {
    variables(); // Now I learned how to do function calls :D
}

fn variables() {
    let mut x = 5;
    println!("x equals {x}");
    x = 6;
    print!("now x equals {x}\n");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);
}
