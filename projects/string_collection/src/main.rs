fn main() {
    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");
    let mut s = String::new();

    let s2 = "bar";
    s.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    //不会获取任何参数的所有权
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    // String 是一个 Vec<u8> 的封装
    // 这里每一个字母的 UTF-8 编码都占用一个字节
    let len = String::from("Hola").len();
    println!("Hola len is {}", len);

    //每个 Unicode 标量值需要两个字节存储
    let len = String::from("Здравствуйте").len();
    println!("Здравствуйте len is {}", len);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
