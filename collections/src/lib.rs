fn main() {
    let mut v: Vec<i32> = Vec::new();
    // let v2 = vec![1,2,3]; 

    v.push(1);
    v.push(2);
    v.push(3);

    let third: &i32 = &v[2];
    println!("an element is {}", third);

    match v.get(2) {
        Some(third) => println!("an element is {}", third),
        None => println!("Nope."),
    }

    for i in &v {
        println!("{}", i)
    }
}
