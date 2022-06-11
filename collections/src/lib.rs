// fn cool_vecs() {
//     let v: Vec<i32> = Vec::new();
//      let v2 = vec![1,2,3];
//
//      v.push(1);
//      v.push(2);
//      v.push(3);
//
//     let third: &i32 = &v[2];
//     println!("an element is {}", third);
//
//     match v.get(2) {
//         Some(third) => println!("an element is {}", third),
//         None => println!("Nope."),
//     };
//
//     for i in &v {
//         println!("{}", i)
//     }
// }

use std::collections::HashMap;

fn cool_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    //
    // let mut scores: HashMap<_, _> =
    //     teams.into_iter().zip(initial_scores.into_iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
}
