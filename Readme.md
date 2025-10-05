# Joke Generator API

## API Endpoints

---

### `GET /jokes/random`

Fetch a random joke.

**Query Parameters:**
- `keyword` (optional): Restrict jokes to a specific type.  
  **Allowed values:** One of the types returned by `/jokes/types` (e.g., `general`, `programming`, `knock-knock`).

**Sample Requests:**
```sh
curl -s "http://127.0.0.1:8000/jokes/random" | jq .
```
**Sample Response:**
```json
{
  "id": 28,
  "type": "programming",
  "setup": "There are 10 types of people in this world...",
  "punchline": "Those who understand binary and those who don't"
}
```

**With Type Filter:**
```sh
curl -s "http://127.0.0.1:8000/jokes/random?keyword=programming" | jq .
```
**Sample Responses:**
```json
{
  "id": 412,
  "type": "programming",
  "setup": "What do you get when you cross a React developer with a mathematician?",
  "punchline": "A function component."
}
```
```json
{
  "id": 380,
  "type": "programming",
  "setup": "What did the Java code say to the C code?",
  "punchline": "You've got no class."
}
```
```json
{
  "id": 16,
  "type": "programming",
  "setup": "What's the object-oriented way to become wealthy?",
  "punchline": "Inheritance"
}
```

**If no joke matches the type:**
```json
{
  "detail": "No joke found with keyword"
}
```

---

### `GET /jokes/type/{type}`

Fetch a random joke from a specific type.

**Path Parameters:**
- `type`: The joke type (e.g., `general`, `programming`, `knock-knock`).

> **Note:** This endpoint does **not** accept a `keyword` parameter.

**Sample Requests:**
```sh
curl -s "http://127.0.0.1:8000/jokes/type/programming" | jq .
```
**Sample Responses:**
```json
{
  "id": 15,
  "type": "programming",
  "setup": "What's the best thing about a Boolean?",
  "punchline": "Even if you're wrong, you're only off by a bit."
}
```
```json
{
  "id": 399,
  "type": "programming",
  "setup": "I just got fired from my job at the keyboard factory.",
  "punchline": "They told me I wasn't putting in enough shifts."
}
```

**If no joke matches the type:**
```json
{
  "detail": "Joke type not found"
}
```

---

### `GET /jokes/types`

List available joke types.

**Sample Request:**
```sh
curl -s "http://127.0.0.1:8000/jokes/types" | jq .
```
**Sample Response:**
```json
[
  "general",
  "programming",
  "knock-knock"
]
```

---

### Error Response Example

If an invalid type or no joke is found:
```json
{
  "detail": "Joke type not found"
}
```

---

## Implementation Steps

### Set Up the Environment

- **Install Rust:**  
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Install Redis locally:**  
  - macOS: `brew install redis`  
  - Ubuntu: `sudo apt install redis`

### Integrate Official Joke API

- Use `reqwest` to fetch jokes from [https://official-joke-api.appspot.com](https://official-joke-api.appspot.com)
- Handle types: `general`, `programming`, `knock-knock`

### Implement Caching

- Use Redis to cache responses for 5 minutes
- Cache by endpoint and query parameters

### Add Filtering

- Filter jokes by type using the `keyword` parameter on `/jokes/random` only

### Document and Test

- Generate OpenAPI docs manually or with `paperclip`
- Test endpoints with `curl` or Postman

---

## Run the Project

1. Build and run the Rust API:
   ```sh
   cargo run
   ```
2. Access the API at [http://localhost:8000](http://localhost:8000)

---

## Run the Flask GUI

1. Install dependencies:
   ```sh
   pip install flask flask_bootstrap requests pync
   ```

2. Run the Flask GUI:
   ```sh
   python gui/app.py
   ```

3. Open your browser and go to:
   ```
   http://127.0.0.1:5000
   ```

<img src="Joke Generator GUI.png" alt="Joke Generator" width="400" />

4. Run the CLI APP:
   ```sh
   python cli/app.py
   ```

<img src="Joke Generator CLI 1.png" alt="Joke Generator" width="400" />

<img src="Joke Generator CLI 2.png" alt="Joke Generator" width="400" />

5. Run the Notifier Script:
```sh
python gui/notifier.py
```

<img src="Joke Notifier.png" alt="Joke Generator" width="400" />

6. Run all Components At Once:
```sh

./setup.sh
./run_all.sh

## Setup Cron Job

crontab -e
@reboot <path_to_[run_all.sh]_script>
```
---

## Best Practices Implemented

- **RESTful Design:**  
  Nouns for endpoints (`/jokes`, `/jokes/random`). Proper HTTP status codes (200, 404, etc.).

- **Caching:**  
  Redis caches responses for 5 minutes to reduce API calls. Cache keys are unique per endpoint and query parameters.

- **Error Handling:**  
  Graceful handling of external API failures with fallback dataset. Clear error messages for invalid types or keywords.

- **Type Safety:**  
  Uses `serde` for safe JSON serialization/deserialization. Strong typing with Rust’s type system.

- **Performance:**  
  Async I/O with Actix Web and reqwest. Type filtering on the server side.

- **Documentation:**  
  Add OpenAPI docs manually or use the `paperclip` crate for Swagger integration.

> This project was casually coded.
>
> Minor Changes: Auto-generated code-level and component-level documentation has been added.
