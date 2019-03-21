
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!(" user1 email is {}", user1.email);
    println!(" user1 username is {}", user1.username);
    println!(" user1 active is {}", user1.active);
    println!(" user1 sign_in_count is {}", user1.sign_in_count);

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        ..user1
    };

    user2.email = String::from("anotheremail@example.com");

    let _user3 = build_user(String::from("builduser@example.com"), String::from("builduser"));
    // 注意 black 和 origin 值的类型不同，因为它们是不同的元组结构体的实例。 
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn build_user(email:String, username:String) -> User {
    User {
        email,
        username,
        active:true,
        sign_in_count:1,
    }
}
