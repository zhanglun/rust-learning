#[derive(PartialEq, Debug)]
struct Car { color: String, motor: Transmission, root: bool, age: (Age, u32) }

#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }


fn main() {
    println!("Hello, world!");
}
