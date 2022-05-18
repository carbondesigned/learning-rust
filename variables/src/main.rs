fn main() {
    /* The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. */
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("the val of x in the inner scope is: {}", x);
    }
    println!("the val of x is : {}", x);

    /* saves time and readability to not have to create separate variables such as spaces_str and spaces_num

    i.e.
        let spaces = "   ";
        let spaces = spaces.len();
    */
}
