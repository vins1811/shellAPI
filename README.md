# shellAPI

Un semplice tool CLI in Rust per generare file FastAPI tramite AI.

## Installazione

1. Clona il repository:
   ```sh
   git clone <repository-url>
   cd shellAPI
   ```

2. Configura la chiave API:
   Crea il file `.cargo/config.toml`:
   ```toml
   [env]
   SHELLAPI_API_KEY = "your-api-key-here" usiamo openrouter con deepseek. API gratuite
   ```

3. Compila il progetto:
   ```sh
   cargo build --release
   ```

4. (Opzionale) Aggiungi l'eseguibile al PATH:
   ```powershell
   $env:Path += ";$PWD\target\release"
   ```

## Uso

Esegui il programma con:
```sh
shellAPI
```

### Menu Interattivo

Il programma presenta un menu interattivo con le seguenti opzioni:

1. **Creare un nuovo file FastAPI**
   - Richiede nome del file
   - Richiede routes (separate da virgola)
   - Richiede metodi HTTP (separati da virgola)

2. **Modificare un file FastAPI esistente**
   - Richiede nome del file esistente
   - Richiede nuove routes (separate da virgola)
   - Richiede nuovi metodi HTTP (separati da virgola)

3. **Generare API con AI**
   - Richiede nome del file da creare
   - Richiede il prompt per l'AI

4. **Modificare API con AI**
   - Richiede nome del file da modificare
   - Richiede il prompt per l'AI

5. **Uscire dal programma**

### Esempi

Dopo aver selezionato l'opzione 1 o 2:
```
Enter file name: test
Enter routes (comma-separated): api1,api2
Enter methods (comma-separated): get,post
```

Questo creerà/modificherà `test.py` con il contenuto:
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

Per l'opzione 3 (Generazione AI):
```
Enter file name: users_api
Enter your prompt: crea un endpoint GET che restituisca un messaggio di benvenuto
```

## Struttura del progetto

- **`main.rs`**
  - Gestisce l'interfaccia CLI interattiva
  - Menu principale e input utente
  - Gestione dello spinner durante le operazioni

- **`commands.rs`**
  - `create(file_name_or_path: &str)`: Crea un file Python con FastAPI
  - `add_route(file_name_or_path: &str, route: &str, method: &str)`: Aggiunge route al file

- **`function.rs`**
  - `is_valid(route: &str, method: &str)`: Verifica validità di route e metodi
  - `validate_add_route(route: &str, method: &str, file_path: &str)`: Validazione completa

- **`ai.rs`**
  - `call_ai_api(prompt: &str)`: Genera codice tramite API OpenRouter
  - `spinner()`: Mostra animazione durante l'elaborazione

- **`build.rs`**
  - Gestisce la configurazione durante la compilazione
  - Incorpora la chiave API nell'eseguibile

## Requisiti

- Rust (ultima versione stabile)
- Python 3.7+ (per eseguire i file generati)
- FastAPI (`pip install fastapi`)
- Uvicorn (`pip install uvicorn`)

## Note

- La chiave API viene incorporata nell'eseguibile durante la compilazione
- Il codice generato dall'AI è ottimizzato per essere direttamente eseguibile
- Tutti i file vengono creati con estensione `.py`
- I metodi HTTP supportati sono: GET, POST, PUT, DELETE
- Dopo ogni operazione, premere Enter per continuare o 'q' per uscire

## Sicurezza

- La chiave API è incorporata nell'eseguibile durante la compilazione
- Il file `.cargo/config.toml` non deve essere incluso nel controllo versione
- Aggiungere `.cargo/config.toml` al `.gitignore`