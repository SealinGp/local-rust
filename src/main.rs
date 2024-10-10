use std::env::var;

// mod life;
// mod limit;
mod utils;

#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {}
}

fn main() {
    ffn();

    let x = String::from("xx");
    let y = String::from("xxx");
    let r = utils::str_helper::longest(x.as_str(), y.as_str());
    println!("{:?}", r)
}

fn ffn() {
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    println!("{:?}", loan);
    foo.share();
}
