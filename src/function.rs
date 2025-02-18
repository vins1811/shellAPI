use std::io::{Read, BufReader};
use crate::commands::add_route;
use std::path::Path;
use std::process::{Command, Stdio};

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

pub fn call_ollama(prompt: &str) {
    println!("Generating the API code...\n");

    // Nuovo contesto in cui si richiede all'AI di produrre codici Python per FastAPI
    let context = "Contesto: Stiamo sviluppando un'API in Python utilizzando FastAPI. L'obiettivo è generare automaticamente codice Python che definisca endpoint REST per supportare operazioni comuni (GET, POST, PUT, DELETE). Il codice generato dovrà includere la validazione dei dati, la gestione degli errori e ritornare risposte in formato JSON. Inoltre, verranno utilizzati moduli come pydantic per la validazione dei dati.";
    
    // Esempi aggiornati in cui l'AI deve produrre codice Python per FastAPI
    let example_1 = r#"Esempio 1:
Input: "Genera un endpoint GET che restituisca un messaggio di benvenuto."
Risposta:
```python
from fastapi import FastAPI
app = FastAPI()

@app.get("/welcome")
async def welcome():
    return {"message": "Benvenuto!"}
```"#;
    
    let example_2 = r#"Esempio 2:
Input: "Genera un endpoint POST per creare un nuovo utente. L'utente ha nome e email."
Risposta:
```python
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

app = FastAPI()

class User(BaseModel):
    name: str
    email: str

@app.post("/users")
async def create_user(user: User):
    # Esegui controlli, ad es. per email duplicata
    return {"message": "Utente creato", "user": user.dict()}
```"#;
    
    let example_3 = r#"Esempio 3:
Input: "Genera un endpoint PUT per aggiornare i dati di un utente esistente."
Risposta:
```python
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

app = FastAPI()

class UserUpdate(BaseModel):
    name: str
    email: str

@app.put("/users/{user_id}")
async def update_user(user_id: int, user: UserUpdate):
    # Verifica se l'utente esiste, altrimenti lancia un'eccezione
    return {"message": "Utente aggiornato", "user": user.dict()}
```"#;
    
    let example_4 = r#"Esempio 4:
Input: "Genera un endpoint DELETE per eliminare un utente in base all'id."
Risposta:
```python
from fastapi import FastAPI, HTTPException

app = FastAPI()

@app.delete("/users/{user_id}")
async def delete_user(user_id: int):
    # Verifica se l'utente esiste, altrimenti restituisci errore 404
    return {"message": "Utente eliminato con successo"}
```"#;

    let complete_prompt = format!(
        "{}\n\n{}\n\n{}\n\n{}\n\n{}\n\nDomanda: {}",
        context, example_1, example_2, example_3, example_4, prompt
    );

    let output = Command::new("ollama")
        .arg("run")
        .arg("llama_FastEvent") // Modifica con il nome corretto del modello, se necessario
        .arg(&complete_prompt)
        .stdout(Stdio::piped()) // Cattura l'output
        .spawn()
        .expect("Errore durante l'esecuzione di ollama")
        .stdout
        .expect("Errore nel recupero dell'output");

    let mut buffer = String::new();
    let mut reader = BufReader::new(output);
    reader
        .read_to_string(&mut buffer)
        .expect("Errore nella lettura dell'output");

    println!("\n--- OUTPUT OLLAMA ---\n");
    println!("{}", buffer);
}

