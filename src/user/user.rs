
#[derive(Debug)]
pub struct User {
    name: String,
    email: String,
    password: String,
    address: String,
    age: i32
}

impl Default for User {
    fn default() -> User {
        User { name: String::from(""), email: String::from(""), password: String::from(""), address: String::from(""), age: 0 }
    }
}

impl User {
    pub fn create_user() -> User {
        let user = User {
            name: String::from("John"),
            email: String::from("John@example.com"),
            password: String::from("test"),
            address: String::from("incheon"),
            age: 32
        };

        user
    }

    pub fn update_user() -> String {
        let mut user: String = String::from("updated");
        user
    }

    pub fn delete_user() -> String {
        let mut user: String = String::from("deleted");
        user
    }

    pub fn search_user() -> String {
        let mut user: String = String::from("search");
        user
    }

    pub fn password_encode() -> String {
        let mut password_encode: String = String::from("password_encode");
        password_encode
    }

    pub fn password_decode() -> String {
        let mut password_decode: String = String::from("password_decode");
        password_decode
    }
}

