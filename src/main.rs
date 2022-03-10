use crate::patterns::observer::{self, IListen, INotify, Observable};
mod patterns;

#[derive(Debug)]
struct SomeType {
    value: i32,
    memes: Vec<String>,
}

fn main() {
    let mut some_type = SomeType {
        value: 1,
        memes: vec!["one".to_string(), "two".to_string()],
    };

    let mut observable: Observable<SomeType> = Observable::new();
    observable.subscribe(observer::Observer::new(|data| println!("{:?}", data)));
    observable.notify(&some_type);
    some_type.value = 42;
    some_type.memes.push("three".to_string());
    observable.notify(&some_type);
}
