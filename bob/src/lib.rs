pub fn reply(message: &str) -> &str {
    let v: Vec<char> = message.trim().chars().collect();
    if v.len() == 0 {
        return "Fine. Be that way!";
    } else if v.ends_with(&['?']) {
        if v.iter().filter(|c| c.is_alphabetic()).count() > 0
            && !v
                .iter()
                .filter(|c| c.is_alphabetic())
                .any(|c| c.is_lowercase())
        {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Sure.";
        }
    } else if v.iter().filter(|c| c.is_alphabetic()).count() > 0
        && !v
            .iter()
            .filter(|c| c.is_alphabetic())
            .any(|c| c.is_lowercase())
    {
        return "Whoa, chill out!";
    } else {
        return "Whatever.";
    }
}
