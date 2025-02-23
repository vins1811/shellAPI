use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{stdout, Write};
use tokio::time::{sleep, Duration};
use std::env;

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

pub async fn call_ai_api(prompt: &str, context_file: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    // Use compile-time API key
    let api_key = env!("SHELLAPI_API_KEY");
    let model = "deepseek/deepseek-chat:free".to_string();
    let url = "https://openrouter.ai/api/v1/chat/completions".to_string();



    let mut context = String::from("Contesto: Stiamo sviluppando un'API in Python utilizzando FastAPI. 
    L'obiettivo è generare automaticamente codice Python che definisca endpoint REST
     per supportare operazioni comuni (GET, POST, PUT, DELETE). 
    Il codice generato dovrà includere la validazione dei dati, la gestione degli errori e ritornare risposte in formato JSON.
     Inoltre, verranno utilizzati moduli come pydantic per la validazione dei dati.
     PRODUCI SOLO CODICE PYTHON PER FASTAPI, NO COMMENTI, SPIEGAZIONI, RAGIONAMENTO. GENERA PLAIN TEXT NO MARKDOWN.");

    if let Some(file_context) = context_file {
        context.push_str("\n\nRiscrivi tutto il codice già presente e aggiungine di nuovo con le nuove 
        esigente che ti ho detto\nCodice già presente: :\n");
        context.push_str(file_context);
    }
    
    let example = include_str!("./example.txt");

    let complete_prompt = format!(
        "{}\n{}",
        example, prompt
    );

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
