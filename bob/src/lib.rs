pub fn reply(message: &str) -> &str {
    enum Message {
        Question,
        Yell,
        QuestionYell,
        Nothing,
        Else,
    }
    // println!("{}\n=====111111222222222{}end", message, message.trim());

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
    // message
}
