use std::fmt::Write;

pub fn verse(n: u32) -> String {
    let mut s = String::new();
    if n == 0 {
        write!(&mut s, "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n").unwrap();
    }
    if n == 1 {
        writeln!(&mut s, "{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n,n).unwrap();
    }
    if n > 1 && n < 100 {
        if n == 2 {
            writeln!(&mut s, "{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n,n, n-1).unwrap();
        } else {
            writeln!(&mut s, "{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n,n, n-1).unwrap();
        }
    }

    s
}

pub fn sing(start: u32, end: u32) -> String {
    let mut s = String::new();
    for i in (end..=start).rev() {
        writeln!(&mut s, "{}\n", verse(i)).unwrap();
    }
    s[0..s.len() - 1].to_string()
}
