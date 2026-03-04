// Structs

// structs can use different types

fn main() {
    // tuple
    let rect: (i32,i32) = (200,500);

    // structs

    struct Book{
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1: User = User{
        active: true,
        username: String::from("someusername"),
        email: String::from("someusername@m.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("changeemail@.com");
    println!("User email is {}",user1.email);


    fn build_user(email: String, username: String) ->User{
        User{
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }

    // create instances from other instances

    let user2: User = User{
        email: String::from("another@m.com"),
        ..user1 // this fills other fields from user1
    };

    // tuple structs

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black: Color = Color(0,0,0);
    let white: Color = Color(255,255,255);

    // unit-like Structs

    struct AlwaysEqual;
    let subject: AlwaysEqual = AlwaysEqual;
}
