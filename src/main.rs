mod day_3 {
    pub mod sol;
}

fn main() {
    let start = std::time::Instant::now();
    day_3::sol::part_1();
    println!("Part 1: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    day_3::sol::part_2();
    println!("Part 2: {:?}", start.elapsed());
}
