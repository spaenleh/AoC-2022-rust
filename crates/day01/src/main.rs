const INPUT: &str = include_str!("input.txt");

fn main() {
    // split on double line breaks
    let mut calorie_sums: Vec<u32> = INPUT
        .split("\n\n")
        .into_iter()
        .map(|elf| {
            elf.split('\n')
                .into_iter()
                .map(|v| match v.trim().parse::<u32>() {
                    Ok(value) => value,
                    Err(_) => 0,
                })
                .sum()
        })
        .collect();
    calorie_sums.sort();
    calorie_sums.reverse();
    let largest_callories: u32 = calorie_sums[0..3].iter().sum();

    println!("largest single {}", calorie_sums[0]);
    println!("largest top3 {}", largest_callories);
}
