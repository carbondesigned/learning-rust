fn main () {
    // Some(T), None are included in the prelude, but just to be clear.
    let some_number: Option<i32> = Option::Some(5);
    let some_number: Option<&str> = Option::Some("this is a string");

    // can't infer None.
    let null_number: Option<i32> = Option::None;

    // you have to convert an Option<T> to a T before you can perform T operations with it. 
    // Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.
    // https://doc.rust-lang.org/std/option/enum.Option.html
}