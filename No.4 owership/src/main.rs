// Rust 中的每一个值都有一个被称为“所有者(ower)的变量”
// 值在任意时刻有且只有一个所有者
// 当所有者(变量)离开作用域，这个值将被丢弃
fn main() {
    println!("Hello, world!");
    let a = String::from("hello"); // 两个冒号是运算符，允许将特定的from函数置于String类型的命名空间

    println!("S {}", a);

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 在字符串后面追加字面值o

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;

    // Rust 则认为 s1 不再有效，因此 Rust 不需要在 s1 离开作用域后清理任何东西
    // println!("{}, world", s1); 
    println!("{}, world", s2); 

    // Clone

    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);



    let str = String::from("hello");    // s 进入作用域

    takes_owership(str);                  // s 的值移动到函数里。。。

    let x = 5;                          // x 进入作用域

    makes_copy(x);                      // x 应该移动到函数里


} // 这里， x会先移出作用域， 然后是s, 但是s的值已经被移走了。 所以不会有特殊操作


fn takes_owership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里， some_integer 移出作用域， 不会有特殊操作