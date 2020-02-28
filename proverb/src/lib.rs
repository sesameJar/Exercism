use std::fmt::Write;

pub fn build_proverb(list: &[&str]) -> String {
    let mut v: Vec<String> = Vec::new();
    let mut s = String::new();
    if list.len() == 0 {
        return String::new();
    } else {
        for (index, word) in list.iter().rev().enumerate() {
            match index {
                0 => {
                    let mut string = String::new();
                    write!(&mut string, "And all for the want of a {}.", list[index]);
                    v.push(string);
                }
                _ => {
                    let mut string = String::new();
                    write!(
                        &mut string,
                        "For want of a {} the {} was lost.",
                        list[index - 1],
                        list[index]
                    );
                    v.push(string);
                }
            }
        }
        let first_item = v.remove(0);
        v.push(first_item);
        v.join("\n")
    }
}
