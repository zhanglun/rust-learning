// mod authentication {
//     use std::collections::hash_map::DefaultHasher;
//     use std::hash::{Hash, Hasher};

//     pub struct User {
//         username: String,
//         password_hash: u64,
//     }

//     impl User {
//         pub fn new(username: &str, password: &str) -> User {
//             User {
//                 username: username.to_string(),
//                 password_hash: hash_password(&password.to_owned()),
//             }
//         }

//         pub fn get_username(&self) -> &String {
//             &self.username
//         }

//         pub fn set_password(&mut self, new_password: &str) {
//             self.password_hash = hash_password(&new_password.to_owned())
//         }
//     }

//     fn hash_password<T: Hash>(t: &T) -> u64 {
//         let mut s = DefaultHasher::new();
//         t.hash(&mut s);
//         s.finish()
//     }
// }

mod authentication;
use regex::Regex;

fn main() {
    println!("Hello, world!");

    let mut user = authentication::User::new("jeremy", "super-secret");

    println!("The username is: {}", user.get_username());

    user.set_password("even-more-secret");
     
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}
