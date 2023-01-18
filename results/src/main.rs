use std::{
    fs::File,
    io::{self, Read}, error::Error,
};

fn main() -> Result<(), Box<dyn Error>>{
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(created_file) => created_file,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };
    // println!("{:#?}", greeting_file);

    let username = read_username_from_file()?;
    println!("{username}");
    // match username {
    //     Ok(username) => println!("Got username: {}", username),
    //     Err(e) => println!("Failed to get username -- {}", e),
    // }
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
