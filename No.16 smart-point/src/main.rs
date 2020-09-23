/*
    Rust 中最常见的指针是引用(reference)，以 & 符号为标识，并借用他们所指向的值。除了引用数据没有任何其他特殊功能
    也没有任何额外开销，所以应用最多

    智能指针是一类数据结构，表现类似指针，但是也拥有额外的元数据和功能。智能指针拥有他们指向的数据
 */
use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::mem::drop; //也可以使用 std::mem::drop 提早丢弃值

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

// 定义了一个结构体 MyBox 并声明了一个泛型参数 T，因为我们希望其可以存放任何类型的值。
// MyBox 是一个包含 T 类型元素的元组结构体。
// MyBox::new 函数获取一个 T 类型的参数并返回一个存放传入值的 MyBox 实例
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Deref trait，由标准库提供，要求实现名为 deref 的方法，其借用 self 并返回一个内部数据的引用
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/** Drop **/
// Drop trait 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用。
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}



fn main() {
    // 定义了变量 b，其值是一个指向被分配在堆上的值 5 的 Box
    let b = Box::new(5);
    println!("b = {}", b);
    println!("Hello, world!");

    // 第一个 Cons 储存了 1 和另一个 List 值。这个 List 是另一个包含 2 的 Cons 值和下一个 List 值。
    // 接着又有另一个存放了 3 的 Cons 值和最后一个值为 Nil 的 List，非递归成员代表了列表的结尾
    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    // 将 Box 放入 Cons 成员中而不是直接存放另一个 List 值。Box 会指向另一个位于堆上的 List 值，而不是存放在 Cons 成员中
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    // &m -> MyBox<String>
    // &MyBox<String> -> &String
    // &String -> &str
    let m = MyBox::new(String::from("Rust"));
    hello(&m);


    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    
    // drop(c);  使用std::mem::drop 显式清理

    println!("CustomSmartPointers created.");

    // 此时到了作用域末尾，c d 离开作用域，Rust调用drop方法
}
