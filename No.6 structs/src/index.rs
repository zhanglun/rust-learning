struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    println!("Hello, world!");
    // let user1 = User {
    //     email: String::from("zhanglun1410@gmail.com"),
    //     username: String::from("zhanglun"),
    //     active: true,
    //     sign_in_count: 1
    // };

    // 报错，因为user1是不可变的，对应的字段也是不可变的
    // user1.email = String::from("548836800@qq.com");

    let mut user1 = User {
        email: String::from("zhanglun1410@gmail.com"),
        username: String::from("zhanglun"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("548836800@qq.com");
    let user2 = build_user(String::from("asdf"), String::from("liuliu"));
    println!("user2: {:?}", user2);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
