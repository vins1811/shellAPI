# shellAPI

Un semplice tool CLI in Rust per generare file FastAPI con percorsi specificati.

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
   setx PATH "%PATH%;C:\Users\<tuo-utente>\OneDrive\Documenti\shellRust\shellAPI\target\release"
   ```
4. Verifica l'installazione eseguendo:
   ```sh
   shellAPI --help
   ```

## Uso

Il programma consente di creare file Python con FastAPI e di aggiungere automaticamente le route specificate.

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

#### **Uscire dal programma**
```sh
shellAPI exit
```

## Struttura del progetto

- **`main.rs`**: Gestisce l'input da terminale e richiama le funzioni appropriate.
- **`commands.rs`**:
  - `create(file_name_or_path: &str)`: Crea un file Python con FastAPI.
  - `add_route(file_name_or_path: &str, route: &str, method: &str)`: Aggiunge route a un file esistente.
- **`functions.rs`**:
  - `is_valid(route: &str, method: &str)`: Verifica che le route e i metodi forniti siano validi.


