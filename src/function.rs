use crate::commands::add_route;
use std::path::Path;

fn is_valid(route: &str, method: &str) -> bool {
    let valid_methods = ["get", "post", "put", "delete"];
    
    if route.starts_with("{") && route.ends_with("}") && 
       method.starts_with("{") && method.ends_with("}") {
        // Extract and split the contents of the braces
        let routes: Vec<&str> = route[1..route.len()-1]
            .split(',')
            .map(|s| s.trim())
            .collect();
            
        let methods: Vec<&str> = method[1..method.len()-1]
            .split(',')
            .map(|s| s.trim())
            .collect();

        // Check if both vectors have the same length
        if routes.len() != methods.len() {
            println!("Error: Number of routes and methods must match!");
            return false;
        }

        // Check if all methods are valid
        for m in methods.iter() {
            if !valid_methods.contains(&m.to_lowercase().as_str()) {
                println!("Method '{}' not valid", m);
                return false;
            }
        }
        
        true
    } else {
        println!("Route or method not valid");
        false
    }
}

pub fn validate_add_route(route: &str, method: &str, file_path: &str) -> bool {
    if !Path::new(file_path).exists() {
        println!("Error: File '{}' does not exist", file_path);
        return false;
    }

    if is_valid(&route, &method) {
        println!("Adding routes...");
        let routes = route.trim_matches(|c| c == '{' || c == '}')
                            .split(',')
                            .map(|s| s.trim());
        
        let methods = method.trim_matches(|c| c == '{' || c == '}')
                            .split(',')
                            .map(|s| s.trim());

        for (route, method) in routes.zip(methods) {
            println!("Adding route: {} with method: {}", route, method);
            add_route(&file_path, route, method);  // Pass file_path directly
        }  

        return true; 
    }
    else {
        println!("Error: Invalid route or method");
        return false;
    }
}



