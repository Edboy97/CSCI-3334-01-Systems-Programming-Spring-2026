use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}

fn main() {
    println!("Welcome to File Operations Program!");
    loop {
        print_menu();
        let choice = get_input("Please enter your choice (0-5): ");
        match choice.as_str(){
            "1" => {
                let path = get_input("Enter the Directory Path: ");
                perform_operation(FileOperation::List(path));
            }
            "2" => {
                let path = get_input("Enter the File Path: ");
                perform_operation(FileOperation::Display(path));
            }
            "3" => {
                let path = get_input("Enter the File Path: ");
                let content = get_input("Enter the content: ");
                perform_operation(FileOperation::Create(path, content));
            }
            "4" => {
                let path = get_input("Enter file path: ");
                perform_operation(FileOperation::Remove(path));
            }
            "5" => {
                perform_operation(FileOperation::Pwd);
            }
            "0" => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("\nInvalid Choice. Please enter a number between the parameters (0-5)"),
        }
        println!();
    }
}
fn print_menu(){
    println!("--- File Operations Menu ---");
    println!("1. List the files in a directory");
    println!("2. Display the file contents");
    println!("3. Create a new file");
    println!("4. Remove a file");
    println!("5. Print the working directory");
    println!("0. Exit");
}
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the line");
    input.trim().to_string()
}
fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            let status = Command::new("ls").arg(path).status();
            if status.is_err() || !status.unwrap().success() {
                eprintln!("Error: Wasn't not able to list Directory.")
            }
        }
        FileOperation::Display(path) => {
            let status = Command::new("cat").arg(path).status();
            if status.is_err() || !status.unwrap().success(){
                eprintln!("Error: Wasn't not able to display file.");
            }
        }
        FileOperation::Create(path, content) => {
            let user_command = format!("echo '{}' > {}", content, path);
            let output = Command::new("sh").arg("-c").arg(&user_command).output().expect("Failled to excute the command");
            if output.status.success() {
                println!("File '{}' created successfully. ", path);
            }
            else {
                eprintln!("Error: Failed to create the file.");
            }
        }
        FileOperation::Remove(path) => {
            let status = Command::new("rm").arg(&path).status();
            if status.is_ok() && status.unwrap().success() {
                println!("File '{}' was removed sucessfully.", path);
            }
            else {
                eprintln!("Error: Failed to reomve the file.");
            }
        }
        FileOperation::Pwd => {
            let status = Command::new("pwd").status();
            if status.is_err() || !status.unwrap().success() {
                eprintln!("Error: Couldn't determine the current Directory.");
            }
        }
    }
}