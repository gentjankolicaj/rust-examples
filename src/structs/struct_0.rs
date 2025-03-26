#[derive(Debug)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

fn main() {
    let month = 1;
    let year = 2025;
    let day = 01;

    let current_date = Date { year, month, day };

    println!("Current date: {:#?}", current_date);
}
