use std::fmt::format;

fn main() {
    let notebook = "first line \n second line \n third line \n \n".to_string();
    let c: Vec<&str> = notebook.split(|c: char|c == '\n').collect();
    println!("notbook lines are {:?}", c);
}