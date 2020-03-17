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
    if v.ends_with(&['?']) {
        if v.iter().any(|x| x.is_lowercase()) {
            return "Sure.";
        }
        return "Calm down, I know what I'm doing!";
    } else if v
        .iter()
        .filter(|x| x.is_alphabetic())
        .all(|c| c.is_uppercase())
    {
        return "Whoa, chill out!";
    } else if v.len()== 0 {
        
        return "Fine. Be that way!";
    } else {
        return "Whatever.";
    }
   
    message
}
