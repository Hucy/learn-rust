fn main() {
    println!("Rust data type");
    scalar_types();
    compound_types();
}

// scalar 标量 
fn scalar_types() {
    integer_type();
    float_type();
    bool_type();
    char_type();
}

// integer 整型
fn integer_type(){
    // i32
    let default_int = -32;
    println!("default_int is i32: {} ",default_int);

    // i8 u8 
    let i8_int:i8 = -8;
    let u8_int:u8 = 8;
    println!("i8_int : {} ",i8_int);
    println!("u8_int : {} ",u8_int);

    // i16 u16
    let i16_int:i16 = -16;
    let u16_int:u16 = 16;
    println!("i16_int : {} ",i16_int);
    println!("u16_int : {} ",u16_int);

    // i32 u32
    let i32_int = -32i32;
    let u32_int:u32 = 0xffu32;
    println!("i32_int : {} ",i32_int);
    println!("u32_int : {} ",u32_int);

    // i64 u64
    let i64_int:i64 = -0o77;
    let u64_int = 0b1111_0000u64;
    println!("i64_int : {} ",i64_int);
    println!("u64_int : {} ",u64_int);

    //isize usize
    let isize_int:isize = -98_222;
    let usize_int:usize = 98_222;
    println!("isize_int : {} ",isize_int);
    println!("usize_int : {} ",usize_int);

    // byte u8 
    let byte_s = b'A';
    println!("byte is u8 only : {} ",byte_s);

    // 整型溢出: debug panic;release:Wrapping( let x:u8 =256; x is 0 )
}

// float 浮点
fn float_type() {
     // f64
    let default_float = -32.01;
    println!("default_int is f64: {} ",default_float);
    
    // f32 (-1)**b31 * (1+sum(b23-i*2**i::1~23) * 2**((b30...b23)-127)
    // https://en.wikipedia.org/wiki/Single-precision_floating-point_format
    let f32_flat:f32 = 32.001;
     println!("f32_flat is : {} ", f32_flat);

    // f64 (-1)**b63 * (1+sum(b52-i*2**i::1~52) * 2**((b62...b52)-127)
    // https://en.wikipedia.org/wiki/Double-precision_floating-point_format
    let f64_flat:f64 = -32.2221;
    println!("f64_flat is: {} ", f64_flat);
}

// bool 布尔
fn bool_type() {
    let t = true;
    let f: bool = false; // 显式指定类型注解
    println!("bool is: {},{}", t,f);
}

// char 字符
fn char_type() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻'; 
    println!("char is: {},{},{}", c,z,heart_eyed_cat);
}


// compound 复合
fn compound_types() {
    tuple_type();
    array_type();
}

// tuple 元组
fn tuple_type(){

    let tup = (500, 6.4, 1);

    // destructuring 解构
    let (_, y, _) = tup;
    println!("The value of y is: {}", y);
    
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    println!("The value of x.0 is: {}", five_hundred);
    
}

// array 数组
fn array_type() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("The value of x[0] is: {}", first);

    let a: [i32; 3] = [1, 2, 3];
    let [_,x,_] = a;
    println!("The value of x is: {}", x);
}