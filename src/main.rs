use std::env;
mod commands;
use commands::{create, add_route};
mod function;
use function::is_valid;

fn commands(args: Vec<String>) -> bool {
    let command = args.get(0).map(|s| s.as_str());
    
    match command {
        Some("exit") => {
            println!("Exiting shellAPI");
            false
        }
        Some("create") => {
            let file_name = args.get(1).map(String::as_str).unwrap_or("main");
            let file_path = if file_name.ends_with(".py") {
                file_name.to_string()
            } else {
                format!("{}.py", file_name)
            };
            
            create(&file_path);
            
            if args.len() >= 4 {
                if is_valid(&args[2], &args[3]) {
                    println!("Adding routes...");
                    let routes = args[2].trim_matches(|c| c == '{' || c == '}')
                                      .split(',')
                                      .map(|s| s.trim());
                    
                    let methods = args[3].trim_matches(|c| c == '{' || c == '}')
                                       .split(',')
                                       .map(|s| s.trim());

                    for (route, method) in routes.zip(methods) {
                        println!("Adding route: {} with method: {}", route, method);
                        add_route(&file_path, route, method);  // Pass file_path directly
                    }   
                }
            }
            true
        }
        _ => {
            println!("Command not recognized");
            true
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Usage: shellAPI <command> [args]");
        println!("Commands:");
        println!("  exit                                    Exit the program");
        println!("  create <file_name> {{routes}} {{methods}}  Create a FastAPI file with routes in the route where you are in the terminal");
        println!("\nExample:");
        println!("shellAPI create test \"{{api1,api2}}\" \"{{get,post}}\"");
        return;
    }

    if !commands(args) {
        std::process::exit(0);
    }
}
