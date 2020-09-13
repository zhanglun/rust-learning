// trait 类似 interface 的概念，但是有点不一样
// trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。
pub trait Summary {
  fn summarize_author(&self) -> String;
  fn summarize(&self) -> String {
      format!("(Read more from {}...)", self.summarize_author())
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize_author(&self) -> String {
      format!("@{}", self.author)
  }

  // 在类中实现 trait 中的方法
  fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize_author(&self) -> String {
      format!("@{}", self.username)
  }

  // fn summarize(&self) -> String {
  //     format!("{}: {}", self.username, self.content)
  // }
}

// 定义一个函数来调用参数 item 上的 summarize 方法
// 参数指定了impl和trait的名称，不是指定类型。参数支持任何实现了指定trait的类型
pub fn notify (item: impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

// `impl Trait` 语法适用于直观的例子，不过他是一个语法糖而已。这个语法称之为 `trait bound`
// 看起来如下
// pub fn notify<T: Summary>(item: T) {
//   println!("Breaking news! {}", item.summarize());
// }

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());

  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
  };

  println!("New article available! {}", article.summarize());
}