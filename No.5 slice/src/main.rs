fn main() {
    // let mut s = String::from("hello world");
    // let word = first_word2(&s);

    // println!("word: {}", word);
    // println!("s: {}", s);

    // s.clear();
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("Hello: {}", hello);
    println!("World: {}", &world[0..1]);
}

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();
    
    println!("bytes: {:?}", bytes);
    println!("b'': {:?}", b'w');

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2 (s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}