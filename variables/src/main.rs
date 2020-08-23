fn main() {
    let x = 5; // 变量默认是不可改变的（immutable）
    println!("The value of x is: {}", x);

    let x = 6; // 使用 let 定义一个与之前变量同名的新变量，而新变量会 隐藏 之前的变量
    println!("The value of x is: {}", x);

    let mut y = 8;  // 使用 mut 允许修改变量绑定的值 ，但是不能修改类型

    println!("Y is: {}", y);
    
    y = 12;

    println!("Y is: {}", y);

    const MAX_POINTS: u32 = 100_000; // Rust 常量的命名规范是使用下划线分隔的大写字母单词，并且可以在数字字面值中插入下划线来提升可读性

    // 不允许对常量使用 mut。常量不光默认不能变，它总是不能变
    // 常量只能被设置为常量表达式，而不能是函数调用的结果，或任何其他只能在运行时计算出的值。

    println!("MAX_POINTS: {}", MAX_POINTS);
}
