pub fn binary_to_decimal(binary: &str) -> u64 {
    let mut decimal: u64 = 0;
    let binary: Vec<char> = binary.chars().rev().collect();

    for i in 0..binary.len() {
        if *binary.get(i).unwrap() == '1' {
            decimal += u64::pow(2, i as u32);
        }
    }

    decimal
}
