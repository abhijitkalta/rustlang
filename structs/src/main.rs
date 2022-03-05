#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn ini(num: u32)-> Square{
        Square{
            width: num,
            height: num
        }
    }
    fn area(&self)-> u32 {
        self.height * self.width 
    }

}

fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        email: String::from("a@gmail.com"),
        active: true,
        sign_in_count: 1,
        username: String::from("abh")
    };
    let user2 = User {
        active: false,
        ..user1
    };
    println!("username for 2: {}", user2.username);
    rec();
    let sqr = Square::ini(5);
    println!("area of sqaure: {}", sqr.area());

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
    let mut s = "Hello world";
    s = "Hello";
    println!("Printing :  {}", s);
    let m = "hello";
    let m1 = &m[0..3];
    println!("{}", m1)

}

fn rec(){
    let rec1 = Rectangle{
        width: 20,
        height: 20
    };
    println!("The area of rectangle is: {}", area(&rec1));
    println!("Rec1 is {:#?}", rec1)
}

fn area(rec: &Rectangle)-> u32 {
    dbg!(rec.height * rec.width)
}

fn plus_one(x: Option<i32>)-> Option<i32> {
    match x {
        None=> None,
        Some(i)=> Some(i+1), 
    }
}
