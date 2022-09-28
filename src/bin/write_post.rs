use std::io::{stdin, Read};

use rust_ddd_sample::*;

use rust_ddd_sample::establish_connection;

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");

    stdin().read_line(&mut title).unwrap();

    let title = title.trim_end();

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );

    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, title, &body);

    println!("\nSaved draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
