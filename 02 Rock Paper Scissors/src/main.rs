mod puzzle;
mod solutions;
fn main() {
    println!("{}", puzzle::PROMPT);
    highlighted(solutions::part_1);
    println!("{}", puzzle::PROMPT_2);
    highlighted(solutions::part_2);
}


fn highlighted<F: Fn()>(f: F) {
    print!("\x1B[7;95m");
    f();
    print!("\x1B[0m");
}
