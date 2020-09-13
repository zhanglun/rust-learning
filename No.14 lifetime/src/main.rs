fn main() {
    println!("Hello, world!");
    // 无法通过编译。r所在作用域大于x的作用域，代码执行到第八行时，x离开作用域
    // 此时 r 无法引用
    // 作用域越大我们就说它 “存在的越久”
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;
    //     }

    //     println!("r: {}", r);
    // }

    // let string1 = String::from("long string is long");

    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is {}", result);
    // }

    /*
        下面的代码无法通过编译
        如果从人的角度读上述代码，我们可能会觉得这个代码是正确的。 
        string1 更长，因此 result 会包含指向 string1 的引用。
        因为 string1 尚未离开作用域，对于 println! 来说 string1 的引用仍然是有效的。
        然而，我们通过生命周期参数告诉 Rust 的是： longest 函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致。
        因此，借用检查器不允许示例 10-24 中的代码，因为它可能会存在无效的引用。
    */
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

/*
    函数签名表名对于某些生命周期'a，函数会获取两个参数，他们都是与生命周期'a存在的一样长的字符串slice
    函数会返回一个同样也与生命周期'a存在一样长的字符串slice
    实际含义是 longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致。
    这就是我们告诉 Rust 需要其保证的约束条件
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
    函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），
    而返回值的生命周期被称为 输出生命周期（output lifetimes）。
    
    Rust 有三个生命周期省略规则
    1. 每一个是引用的参数都有它自己的生命周期参数。换句话说就是，有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，有两个引用参数的函数有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推
    2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。
    3. 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method)， 那么所有输出生命周期参数被赋予 self 的生命周期
*/