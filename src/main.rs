fn main() {
    let s = String::from("abc");
    takes_ownership(s);

    let i: i32 = 1;
    makes_copy(i);
}

fn takes_ownership(s: String) {
    println!("task_ownership:{}", s)
}

fn makes_copy(s: i32) {
    println!("makes_copy:{}", s)
}
