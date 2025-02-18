use std::env;
mod commands;
use commands::create;
mod function;
use function::{validate_add_route, call_ollama, spinner};

// La funzione commands è stata resa asincrona
async fn commands(args: Vec<String>) -> bool {
    let command = args.get(0).map(|s| s.as_str());
    
    match command {
        Some("exit") => {
            println!("Exiting shellAPI");
            false
        }
        Some("create") => {
            if args.len() == 4 {
                let file_name = args.get(1).map(String::as_str).unwrap_or("main");
                let file_path = if file_name.ends_with(".py") {
                    file_name.to_string()
                } else {
                    format!("{}.py", file_name)
                };
                
                create(&file_path);
                validate_add_route(&args[2], &args[3], &file_path);
            }
            true
        }
        Some("modify") => {
            if args.len() == 4 {
                let file_name = args.get(1).map(String::as_str).unwrap_or("main");
                let file_path = if file_name.ends_with(".py") {
                    file_name.to_string()
                } else {
                    format!("{}.py", file_name)
                };
            
                if !validate_add_route(&args[2], &args[3], &file_path) {
                    return true;
                }
            }
            true
        }
        Some("create_AI") => {
            if args.len() == 2 {
                let prompt = args.get(1).unwrap();

                if prompt.is_empty() {
                    println!("Prompt vuoto. Uscita.");
                    return false;
                };

                println!("Generating the API code...\n");

                // Avvia lo spinner in un task separato
                let spinner_handle = tokio::spawn(spinner());

                // Attendi il completamento di call_ollama
                call_ollama(prompt).await;
                spinner_handle.abort();
                true
            } else {
                println!("Usage: shellAPI create_AI <prompt>");
                true
            }
        }
        _ => {
            println!("Command not recognized");
            true
        }
    }
}

// La funzione main deve essere asincrona per utilizzare await
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Usage: shellAPI <command> [args]");
        println!("Commands:");
        println!("  exit\t\t\t\tExit the program");
        println!("  create <file_name> {{routes}} {{methods}}\tCreate a FastAPI file with routes");
        println!("\nExample:");
        println!("shellAPI create test \"{{api1,api2}}\" \"{{get,post}}\"");
        return;
    }

    if !commands(args).await {
        std::process::exit(0);
    }
}
