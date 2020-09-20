/*
    Rust 中最常见的指针是引用(reference)，以 & 符号为标识，并借用他们所指向的值。除了引用数据没有任何其他特殊功能
    也没有任何额外开销，所以应用最多

    智能指针是一类数据结构，表现类似指针，但是也拥有额外的元数据和功能。智能指针拥有他们指向的数据
 */
use crate::List::{Cons, Nil};
fn main() {
    // 定义了变量 b，其值是一个指向被分配在堆上的值 5 的 Box
    let b = Box::new(5);
    println!("b = {}", b);
    println!("Hello, world!");

    // 第一个 Cons 储存了 1 和另一个 List 值。这个 List 是另一个包含 2 的 Cons 值和下一个 List 值。
    // 接着又有另一个存放了 3 的 Cons 值和最后一个值为 Nil 的 List，非递归成员代表了列表的结尾
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}