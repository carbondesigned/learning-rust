struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32,i32,i32);

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {
        // ...
    }
}

fn main() {
    //The body of the method would use self to get the value that we called the method on. 
    // In this example, we’ve created a variable m that has the value Message::Write(String::from("hello")), 
    // and that is what self will be in the body of the call method when m.call() runs.
    let m = Message::Write(String::from("hello"));
    (m);
}