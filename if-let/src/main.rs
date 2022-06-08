fn main() {
    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The max is configured to be {}", max),
    //     _ => (),
    // }

    // actually way easier and clearer way with "if let". -- basically syntax sugar for match
    // It works the same way as a match, where the expression is given to the match and the pattern is its first arm. 
    // In this case, the pattern is Some(max), and the max binds to the value inside the Some. 
    // We can then use max in the body of the if let block in the same way as we used max in the corresponding match arm. 
    // The code in the if let block isn’t run if the value doesn’t match the pattern.
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The max is configured to be {}", max)
    }
}
