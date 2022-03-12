fn main() {
    let s1 = String::from("yellow");

    let s2 = referred_s1(&s1);

    println!("s1: {}", s1);

    println!("s2: {}", s2);
}

fn referred_s1(some_string: &String) -> usize {
    some_string.len()
}
