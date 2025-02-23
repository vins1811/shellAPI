use std::fs::OpenOptions;
use std::io::Write;
use crate::ai::call_ai_api;

// Create api file and write on it import and instantiate FastAPI object named app
fn write_on_file(file_name_or_path: &str, content: &str) {
    std::fs::write(file_name_or_path, content).expect("Error writing to file");
}

pub fn create(file_name_or_path: &str, content: &str) {
    println!("Creating Python file...");

    let _file = std::fs::File::create(&file_name_or_path).expect("Error creating file");
    println!("Python file created successfully!");
    
    write_on_file(&file_name_or_path, &content);
}

// Writes on file routes mapped to the method get, post, put, delete
pub fn add_route(file_name_or_path: &str, route: &str, method: &str) {
    let content = format!("\n@app.{}(\"/{}\")\nasync def {}():\n\tpass\n", 
                        method, route, route.replace("/", "_"));
    
    let mut file = OpenOptions::new()
        .append(true)
        .open(&file_name_or_path)
        .expect("Error opening file");
    
    file.write_all(content.as_bytes())
        .expect("Error writing to file");
    
    println!("Route added successfully to {}", file_name_or_path);
}

pub async fn create_project(project_name: &str, prompt: &str) {
    // CREATING PROJECT STRUCTURE
    println!("Creating Python project...");
    std::fs::create_dir(project_name).expect("Error creating project");

    let mut file = format!("{}/main.py", project_name);
    std::fs::File::create(&file).expect("Error creating main file");

    file = format!("{}/db", project_name);
    std::fs::create_dir(&file).expect("Error creating project");

    file = format!("{}/db/model.py", project_name);
    std::fs::File::create(&file).expect("Error creating model file");

    file = format!("{}/db/schema.py", project_name);
    std::fs::File::create(&file).expect("Error creating schema file");

    file = format!("{}/db/db.py", project_name);
    std::fs::File::create(&file).expect("Error creating db file");
    
    println!("Python project created successfully!\nInitializing project...");

    // INITIALIZING PROJECT
    let files = vec![
        (format!("./{}/main.py", project_name), "main"),
        (format!("./{}/db/model.py", project_name), "model"),
        (format!("./{}/db/schema.py", project_name), "schema"),
        (format!("./{}/db/db.py", project_name), "db"),
    ];

    for (file_path, file_type) in files {
        match call_ai_api(&format!("Create content for {} file in a FastAPI project about {}", file_type, prompt), None, Some(file_type)).await {
            Ok(content) => {
                write_on_file(&file_path, &content);
                println!("{} file initialized successfully!", file_type);
            }
            Err(e) => {
                println!("Error initializing {} file: {}", file_type, e);
            }
        }
    }
}