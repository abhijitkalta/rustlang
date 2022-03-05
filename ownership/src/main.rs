fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}, world", s2);
    let s3 = String::from("hi");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4)
}
