fn main() {
    let year1993 = 1993;
    print_ten_years_ago(year1993);

    let year2021 = 2021;
    println!("{}: 10 years ago was {}", year2021, year2021 - 10);
}

fn print_ten_years_ago(year1993: i32) {
    println!("{}: 10 years ago was {}", year1993, year1993 - 10);
}