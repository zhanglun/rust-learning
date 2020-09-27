use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // 创建一个新线程，传递一个闭包
    // 返回值类型是 JoinHandle。
    // JoinHandle 是一个拥有所有权的值，当对其调用 join 方法时，它会等待其线程结束
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            // 强制线程停止执行一小段时间，这会允许其他不同的线程运行
            thread::sleep(Duration::from_millis(1))
        }
    });

    // 主线程
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("Hello, world!");

    // 调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束
    // 在这里，当主线程执行到这里是，会被阻塞至到handle所代表的线程结束
    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle2.join().unwrap();

    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        
        // send 函数获取其参数的所有权并移动这个值归接收者所有 所以下面不能再 println val
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
