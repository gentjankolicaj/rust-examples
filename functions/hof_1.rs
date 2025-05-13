fn is_even(i: &u32) -> bool {
    *i % 2 == 0
}
fn main() {
    let sum_even: u32 = (0..10).filter(is_even).sum();
    println!("sum_even={}", sum_even);
}
