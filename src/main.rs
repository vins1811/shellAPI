use std::io::{self, Write};
mod commands;
use commands::{create, create_project};
mod function;
use function::validate_add_route;
mod ai;
use ai::{call_ai_api, spinner};

fn continue_prompt() -> bool {
    let choice = get_user_input("\nPress Enter to continue or 'q' to quit: ");
    !choice.eq_ignore_ascii_case("q")
}

#[tokio::main]
async fn main() {
    loop {
        print_menu();
        let choice = get_user_input("Enter your choice: ");

        match choice.as_str() {
            "1" => {
                let file_name = get_user_input("Enter file name: ");
                let routes = get_user_input("Enter routes (comma-separated): ");
                let methods = get_user_input("Enter methods (comma-separated): ");
                
                let file_path = if file_name.ends_with(".py") {
                    file_name
                } else {
                    format!("{}.py", file_name)
                };

                let content = "from fastapi import FastAPI\napp = FastAPI()\n";
                create(&file_path, content);
                validate_add_route(&format!("{{{}}}", routes), &format!("{{{}}}", methods), &file_path);
                
                if !continue_prompt() {
                    break;
                }
            }
            "2" => {
                let file_name = get_user_input("Enter file name: ");
                let routes = get_user_input("Enter routes (comma-separated): ");
                let methods = get_user_input("Enter methods (comma-separated): ");
                
                let file_path = if file_name.ends_with(".py") {
                    file_name
                } else {
                    format!("{}.py", file_name)
                };

                validate_add_route(&format!("{{{}}}", routes), &format!("{{{}}}", methods), &file_path);
                
                if !continue_prompt() {
                    break;
                }
            }
            "3" => {
                let file_name = get_user_input("Enter file name: ");
                let prompt = get_user_input("Enter your prompt: ");
                
                let file_path = if file_name.ends_with(".py") {
                    file_name
                } else {
                    format!("{}.py", file_name)
                };

                println!("Generating the API code...\n");
                let spinner_handle = tokio::spawn(spinner());

                match call_ai_api(&prompt, None, None).await {
                    Ok(generated_code) => {
                        spinner_handle.abort();
                        print!("\x1B[2J\x1B[1;1H");
                        create(&file_path, &generated_code);
                        println!("Code generated successfully!");
                    }
                    Err(e) => {
                        spinner_handle.abort();
                        println!("Error generating code: {}", e);
                    }
                }
                
                if !continue_prompt() {
                    break;
                }
            }
            "4" => {
                let file_name = get_user_input("Enter file name: ");
                let prompt = get_user_input("Enter your prompt: ");
                
                let file_path = if file_name.ends_with(".py") {
                    file_name
                } else {
                    format!("{}.py", file_name)
                };

                let content = std::fs::read_to_string(&file_path)
                    .unwrap_or_else(|_| String::from(""));

                println!("Generating the API code...\n");
                let spinner_handle = tokio::spawn(spinner());

                match call_ai_api(&prompt, Some(&content), None).await {
                    Ok(generated_code) => {
                        spinner_handle.abort();
                        print!("\x1B[2J\x1B[1;1H");
                        create(&file_path, &generated_code);
                        println!("Code generated successfully!");
                    }
                    Err(e) => {
                        spinner_handle.abort();
                        println!("Error generating code: {}", e);
                    }
                }
                
                if !continue_prompt() {
                    break;
                }
            }
            "5" => {
                let proj_name = get_user_input("Enter file name: ");
                let prompt = get_user_input("Enter your prompt: ");
                create_project(&proj_name, &prompt).await;

                if !continue_prompt() {
                    break;
                }
            }
            "6" => {
                println!("Exiting program...");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
                if !continue_prompt() {
                    break;
                }
            }
        }
    }
}

fn print_menu() {
    print!("\x1B[2J\x1B[1;1H");
    println!("=== ShellAPI Menu ===");
    println!("1. Create new FastAPI file");
    println!("2. Modify existing FastAPI file");
    println!("3. Generate API with AI");
    println!("4. Modify API with AI");
    println!("5. Create FastAPI project [MVC configuration]");
    println!("6. Exit");
    println!("===================");
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
