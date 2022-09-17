fn main() {
    let mut str = String::from("Hello My Name is Musab!");
    let first_word_len = first_word(&str);
    str.clear();
    println!("{first_word_len}");
}

fn first_word (s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// int *arr = &array