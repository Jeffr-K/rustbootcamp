// use crate::user::user::User;

mod user;
use user::user::User;

fn main() {
    println!("Hello, world!");

    let mut created: User = User::create_user();
    let mut updated: String = User::update_user();
    let mut deleted: String = User::delete_user();
    let mut searched: String = User::search_user();

    println!("{:?}", created);
    println!("{}", updated);
    println!("{}", deleted);
    println!("{}", searched);
}
