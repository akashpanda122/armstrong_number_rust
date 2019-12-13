pub fn is_armstrong_number(num: u32) -> bool {
    let len = ((num + 1) as f32).log10().ceil() as u32;
    let strigified = num.to_string();
    let armstrong: u32 = strigified.chars()
        .map(|digit| digit.to_digit(10).unwrap())
        .map(|digit| digit.pow(len))
        .sum();
    armstrong == num
}
