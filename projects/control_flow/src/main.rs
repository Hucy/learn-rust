fn main() {
    if_flow();
    loop_flow();
    while_flow();
    for_flow();
    n_fib(14);
}


fn if_flow() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2, 3 or 4");
    }

    // if  做为表达式，返回值
    let condition = true;
    let number = if condition{
        5
    } else {
        6
    };
    println!("number is {}", number);
}


fn loop_flow() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("break counter is {}", result);
}

fn while_flow() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF");
}


fn for_flow() {
    let a = [10, 20, 30, 1, 2];
    for element in a.iter(){
        println!("the value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF");
}


fn n_fib(n:u32) {
    let mut i:u32 = 0;
    while i <= n {
        let fib_num = fib(i);
        println!("{} fib num is {}", i,fib_num);
        i += 1;
    }
}

fn fib(n:u32) -> u32 {
    let limit:u32 = 2;

    if n < limit {
     return  n;
    };

   fib(n-2) + fib(n-1)
}
