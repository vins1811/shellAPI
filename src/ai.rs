use reqwest::Client;
use serde::{Deserialize, Serialize};

use std::io::{stdout, Write};
use tokio::time::{sleep, Duration};
use dotenv::dotenv;

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}

pub async fn call_ai_api(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let context = "Contesto: Stiamo sviluppando un'API in Python utilizzando FastAPI. 
    L'obiettivo è generare automaticamente codice Python che definisca endpoint REST
     per supportare operazioni comuni (GET, POST, PUT, DELETE). 
    Il codice generato dovrà includere la validazione dei dati, la gestione degli errori e ritornare risposte in formato JSON.
     Inoltre, verranno utilizzati moduli come pydantic per la validazione dei dati.
     PRODUCI SOLO CODICE PYTHON PER FASTAPI, NO COMMENTI, SPIEGAZIONI, RAGIONAMENTO. GENERA PLAIN TEXT NO MARKDOWN.";
    
    // Esempi aggiornati in cui l'AI deve produrre codice Python per FastAPI
    let example_1 = r#"Esempio 1:
Input: "Genera un endpoint GET che restituisca un messaggio di benvenuto."
Risposta:
from fastapi import FastAPI
app = FastAPI()

@app.get("/welcome")
async def welcome():
    return {"message": "Benvenuto!"}"#;
    
    let example_2 = r#"Esempio 2:
Input: "Genera un endpoint POST per creare un nuovo utente. L'utente ha nome e email."
Risposta:
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

app = FastAPI()

class User(BaseModel):
    name: str
    email: str

@app.post("/users")
async def create_user(user: User):
    # Esegui controlli, ad es. per email duplicata
    return {"message": "Utente creato", "user": user.dict()}"#;
    
    let example_3 = r#"Esempio 3:
Input: "Genera un endpoint PUT per aggiornare i dati di un utente esistente."
Risposta:
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

app = FastAPI()

class UserUpdate(BaseModel):
    name: str
    email: str

@app.put("/users/{user_id}")
async def update_user(user_id: int, user: UserUpdate):
    # Verifica se l'utente esiste, altrimenti lancia un'eccezione
    return {"message": "Utente aggiornato", "user": user.dict()}"#;
    
    let example_4 = r#"Esempio 4:
Input: "Genera un endpoint DELETE per eliminare un utente in base all'id."
Risposta:
from fastapi import FastAPI, HTTPException

app = FastAPI()

@app.delete("/users/{user_id}")
async def delete_user(user_id: int):
    # Verifica se l'utente esiste, altrimenti restituisci errore 404
    return {"message": "Utente eliminato con successo"}"#;

    let complete_prompt = format!(
        "{}\n\n{}\n\n{}\n\n{}\n\nDomanda: {}",
        example_1, example_2, example_3, example_4, prompt
    );

    dotenv().ok();

    let api_key = "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";
    let model = "deepseek/deepseek-chat:free".to_string(); // Default: gpt-4-turbo
    let url = "https://openrouter.ai/api/v1/chat/completions".to_string();

    let client = Client::new();
    let request_body = OpenAIRequest {
        model,
        messages: vec![
            Message { role: "system".to_string(), content: context.to_string() },
            Message { role: "user".to_string(), content: complete_prompt.to_string() },
        ],
        max_tokens: 700,
        temperature: 0.7,
    };


    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "")  // Obbligatorio per OpenRouter
        .header("X-Title", "") // Nome app arbitrario
        .json(&request_body)
        .send()
        .await?;

    let response_text = response.text().await?;

    // Controlla se la risposta è valida JSON
    let response_body: OpenAIResponse = serde_json::from_str(&response_text)?;

    let generated_text = response_body
        .choices
        .get(0)
        .map_or("No response generated", |choice| &choice.message.content);

    Ok(generated_text.to_string())
}

pub async fn spinner() {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let mut i = 0;
    loop {
        print!("\r{} ", frames[i % frames.len()]);
        stdout().flush().unwrap();
        sleep(Duration::from_millis(80)).await;
        i += 1;
    }
}
