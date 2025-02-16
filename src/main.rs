use std::env;
use std::io::{stdin, stdout, Write};
mod licenses;

fn license() {
    let licenses: Vec<&str> = vec!["mit", "defec"];

    println!("License Generator");
    println!("What license do you want?");
    print!("(type 'list' to output licenses): ");
    let mut input: String = String::new();
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut input);
    match input.trim() {
        "list" => {
            println!("Available Licenses:");
            for keyword in licenses {
                println!("{}", keyword);
            }
            println!("");
            println!("");
            license()
        }
        "mit" => {
            licenses::mit::gen();
        }
        "defec" => {
            licenses::defec::gen();
        }
        _ => {
            println!("\x1b[31mThe license you entered has not been added yet.");
            println!("(Maybe you have the wrong keyword. Please check the list.)\x1b[m");
        }
    }
}
fn main() {
    let version: String = "v1.0.0".to_string();
    println!("Risen version: \x1b[35m{}\x1b[m", version);
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let gen_type: String = args[1].to_ascii_lowercase();
        if &gen_type == "license" {
            license()
        } else {
        }
    }
}
