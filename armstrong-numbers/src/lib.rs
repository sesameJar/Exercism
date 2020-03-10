pub fn is_armstrong_number(num: u32) -> bool {
    let  v: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    num == v.iter().map(|x| x.pow(v.len() as u32)).sum()
}
