
// a struct for a user profile
struct User {
    username: String,
    email: String,
    age: u8,
    active: bool,
}

// Implement methods
impl User {
    fn new(username: &str, email: &str, age: u8, active: bool) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_string(),
            age,
            active,
        }
    }

    fn display(&self) {
        println!(
            "User: {} (Email: {}, Age: {}, Active: {})",
            self.username, self.email, self.age, self.active
        );
    }

    fn activate(&mut self) {
        self.active = true;
        println!("{} is now active.", self.username);
    }

    fn deactivate(&mut self) {
        self.active = false;
        println!("{} is now inactive.", self.username);
    }
}

fn main() {
    let mut user1 = User::new("Alex", "alex@base.com", 30, false);
    user1.display();
    
    user1.activate();
    user1.display();
    
    // Creating another user with struct update syntax
    let mut user2 = User {
        username: "Kim".to_string(),
        ..user1
    };
    
    user2.display();
    user2.deactivate();
    user2.display();
}
