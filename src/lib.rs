mod generator;
pub fn print_random_number() {
    let n = generator::get_run();
    println!("Random u8:{}", n);
}
