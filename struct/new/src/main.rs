struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn activate(&mut self) {
        self.active = true;
    }

    fn print(&self) {
        println!("User Name: {}", self.username);
        println!("Email: {}", self.email);
        println!("Uri: {}", self.uri);
        println!("Active: {}", self.active);
    }
}

fn main() {
    let mut user = User::new(
        String::from("anna"),
        String::from("anna@company.com"),
        String::from("https://about.me/anna"),
    );
    println!("\n# New User");
    user.print();

    println!("\n# Deactivate");
    user.deactivate();
    user.print();

    println!("\n# Activate");
    user.activate();
    user.print();

    println!("\n# Deactivate manually");
    user.active = false;
    user.print();
}
