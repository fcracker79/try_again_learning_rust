#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn disable(&mut self) {
        self.active = false;
    }

    fn get_stuff(self) -> String {
        self.username
    }

    fn build_user(email: &str, username: &str) -> User {
        let email = String::from(email);
        let username = String::from(username);
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

}

fn copy(username: String, user: User) -> User {
    User {
        username,
        ..user
    }
}

fn copy_color(color: Color) -> Color {
    Color {..color}
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("Hello, {:?}!", user1);

    let mut user2 = user1;
    user2.email = String::from("someoneelse@example.com");
    println!("Another user {:?}", user2);
    println!(
        "Not enough of users, here it is another one: {:?}",
        User::build_user("third@user.com", "thirduser")
    );

    let copied_user = copy(String::from("a different user"), user2);
    println!("{:?}", copied_user);

    let black = Color(1, 2, 3);
    println!("Black: {:?}", black);
    let black2 = copy_color(black);
    println!("Black2: {:?}", black2);

    let mut u = User::build_user("email1@email.com", "username1");
    u.disable();
    println!("Disabled user: {:?}", u);
    println!("{}", u.get_stuff());
}
