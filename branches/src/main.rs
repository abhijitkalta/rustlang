fn main() {
    println!("Hello, world!");
    let number: i32 = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4")
    } else if number % 3 == 0 {
        println!("Number is divisible by 3")
    } else if number % 2 == 0 {
        println!("Number is divisible by 2")
    } else {
        println!("number is no divisible by any")
    }
    test();
    loop_test();
    loop_return();
    for_test();
}

fn test() {
    //testing if condition
    let condition: bool = true;
    let number = if condition { 10 } else { 5 };
    println!("{}", number)
}

fn loop_test() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            } 
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1
        }
        count += 1;
    }
    println!("end count = {}", count)
}

fn loop_return() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("counter: {}", result)
}

fn for_test() {
    for i in 1..10 {
        println!("number: {}", i)
    }
}
