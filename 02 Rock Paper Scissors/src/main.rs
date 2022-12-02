mod puzzle;
mod part_1;
mod part_2;
fn main() {
    println!("{}", puzzle::PROMPT);
    part_1::part_1();
    part_2::part_2();
}
