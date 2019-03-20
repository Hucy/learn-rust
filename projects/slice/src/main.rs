fn main() {
    let s = String::from("Hello, world!");
    let word = first_word(&s);
    println!("the first word is {}",word);

    let my_string_literal = "hello world";

    // first_word 中传入 `String` 的 slice
    let word = str_first_word(&s[..]);
    println!("the first word is {}",word);

     // first_word 中传入字符串字面值的 slice
    let word = str_first_word(&my_string_literal[..]);
    println!("the first word is {}",word);
    
    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = str_first_word(my_string_literal);
    println!("the first word is {}",word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn str_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}