# shellAPI

Un semplice tool CLI in Rust per generare file FastAPI tramite AI.

## Installazione

1. Clona il repository:
   ```sh
   git clone <repository-url>
   cd shellAPI
   ```
2. Compila il progetto:
   ```sh
   cargo build --release
   ```
3. Aggiungi l'eseguibile alla variabile di ambiente `Path` (Windows):
   ```sh
   setx PATH "%PATH%;<percorso_del_tuo_progetto>"
   ```
4. Verifica l'installazione eseguendo:
   ```sh
   shellAPI --help
   ```

## Uso

Il programma consente di:
- Creare file Python con FastAPI
- Aggiungere automaticamente route specificate
- Generare codice API tramite AI
- Modificare file esistenti

### Comandi disponibili

#### **Creare un file FastAPI con percorsi e metodi**
```sh
shellAPI create <file_name> "{route1,route2}" "{method1,method2}"
```
Esempio:
```sh
shellAPI create test "{api1,api2}" "{get,post}"
```
Questo comando creerà `test.py` con il contenuto:
```python
from fastapi import FastAPI
app = FastAPI()

@app.get("/api1")
async def api1():
    pass

@app.post("/api2")
async def api2():
    pass
```

#### **Modificare un file FastAPI esistente**
```sh
shellAPI modify <file_name> "{route1,route2}" "{method1,method2}"
```

#### **Generare codice API tramite AI**
Il comando `create_AI` genera automaticamente codice Python per FastAPI basato su un prompt fornito.
Il codice generato sarà **solo codice Python**, pronto per essere salvato ed eseguito.

```sh
shellAPI create_AI "<prompt>"
```
Esempio:
```sh
shellAPI create_AI "Genera un endpoint GET che restituisca un messaggio di benvenuto"
```

#### **Uscire dal programma**
```sh
shellAPI exit
```

## Struttura del progetto

- **`main.rs`**
  - Gestisce l'input da terminale
  - Dispatch dei comandi
  - Integrazione con l'AI
  - Animazione dello spinner durante l'attesa

- **`commands.rs`**
  - `create(file_name_or_path: &str)`: Crea un file Python con FastAPI
  - `add_route(file_name_or_path: &str, route: &str, method: &str)`: Aggiunge route al file

- **`function.rs`**
  - `is_valid(route: &str, method: &str)`: Verifica validità di route e metodi
  - `validate_add_route(route: &str, method: &str, file_path: &str)`: Validazione completa
  - `call_ollama(prompt: &str)`: Genera codice tramite AI
  - `spinner()`: Mostra animazione durante l'elaborazione

## Requisiti

- Rust (ultima versione stabile)
- Ollama installato e configurato
- Modello ShellAI disponibile per ollama
- Python 3.7+ (per eseguire i file generati)
- FastAPI (`pip install fastapi`)
- Uvicorn (`pip install uvicorn`)

## Note

- Il comando `ollama` deve essere disponibile nel PATH
- Il codice generato dall'AI è ottimizzato per essere direttamente eseguibile
- Tutti i file vengono creati con estensione `.py`
- I metodi HTTP supportati sono: GET, POST, PUT, DELETE