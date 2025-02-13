use std::fs::OpenOptions;
use std::io::Write;

//Create api file and write on it import and instanciate FastAPI object named app
fn write_on_file(file_name_or_path: &str, content: &str) {
    std::fs::write(file_name_or_path, content).expect("Error writing to file");
}

pub fn create(file_name_or_path: &str) {
    println!("Creating Python file...");

    let _file = std::fs::File::create(&file_name_or_path).expect("Error creating file");
    println!("Python file created successfully!");
    
    let content = "from fastapi import FastAPI\napp = FastAPI()\n";
    write_on_file(&file_name_or_path, &content);
}

//writes on file routes mapped to the method get, post, put, delete
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