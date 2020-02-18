pub fn raindrops(n: u32) -> String {
    let mut a = String::from("");
    match n {
        n if (n % 3 == 0 && n % 5 !=0 && n % 7 !=0) => a.push_str("Pling"),
        n if (n % 3 != 0 && n % 5 ==0 && n % 7 !=0) => a.push_str("Plang"),
        n if (n % 3 != 0 && n % 5 !=0 && n % 7 ==0) => a.push_str("Plong"),
        n if (n % 3 == 0 && n % 5 ==0 && n % 7 !=0) => a.push_str("PlingPlang"),
        n if (n % 3 == 0 && n % 5 !=0 && n % 7 ==0) => a.push_str("PlingPlong"),
        n if (n % 3 != 0 && n % 5 ==0 && n % 7 ==0) => a.push_str("PlangPlong"),
        n if (n % 3 == 0 && n % 5 == 0  && n % 7 == 0) => a.push_str("PlingPlangPlong"),
        n => a = n.to_string(),
    }
    a
}
