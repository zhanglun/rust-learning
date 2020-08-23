fn main() {
    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    // Rust 期望一个 bool 却得到了一个整数。不像 Ruby 或 JavaScript 这样的语言，Rust 并不会尝试自动地将非布尔值转换为布尔值。必须总是显式地使用布尔值作为 if 的条件
    // let number = 3;
    // if number { 
    //     println!("number was three");
    // }

    // 因为 if 是一个表达式，我们可以在 let 语句的右侧使用它
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    let mut counter = 0; // 声明了一个名为 counter 的变量并初始化为 0
    let result = loop { //声明了一个名为 result 来存放循环的返回值
        counter += 1;

        if counter == 10 {
            break counter * 2; // 使用 break 关键字返回值 counter * 2
        }
    };

    println!("The result is {}", result);

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..=4).rev() { // 内置标准库 Range 生成从搞一个数字开始到另一个数字之前结束的所有数字，(1..4) 1~3不包含4 (1..=4) 1~4 包含4
        println!("{}!", number);
    }
    
    println!("LIFTOFF!!!");
}
