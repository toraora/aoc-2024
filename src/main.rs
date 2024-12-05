mod day_4 {
    pub mod sol;
}

fn main() {
    let start = std::time::Instant::now();
    day_4::sol::part_1();
    println!("Part 1: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    day_4::sol::part_2();
    println!("Part 2: {:?}", start.elapsed());
}
