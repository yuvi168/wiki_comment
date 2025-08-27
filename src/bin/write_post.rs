use demo::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();
    
    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();
    
    println!("\nOk! Let's write {title} (Press {EOF} when
    finished)\n",);
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, &title, &body);
    println!("\nThe post has been saved with the id: {}\n", post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";
