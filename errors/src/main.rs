fn main() {
    // When the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.
    let so_big_of_a_vec = vec![1,2,3];

    let _v = so_big_of_a_vec[99];
}
