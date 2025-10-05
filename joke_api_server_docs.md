# Joke API Server

## Overview
A REST API server built with Actix-web that provides joke data from external APIs with Redis caching. The server fetches jokes from the Official Joke API and includes fallback mechanisms for offline functionality.

## Features
- RESTful API endpoints for jokes
- External API integration (Official Joke API)
- Redis caching for improved performance
- Fallback joke dataset for offline operation
- Keyword-based joke filtering
- JSON response format
- Comprehensive error handling

## Dependencies
```toml
[dependencies]
actix-web = "4.0"
serde = { version = "1.0", features = ["derive"] }
reqwest = { version = "0.11", features = ["json"] }
redis = "0.23"
once_cell = "1.19"
anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Architecture

### Core Components

#### Joke Model
```rust
#[derive(Serialize, Deserialize, Clone)]
struct Joke {
    id: i32,
    #[serde(rename = "type")]
    joke_type: String,
    setup: String,
    punchline: String,
}
```

#### Error Response
```rust
#[derive(Serialize)]
struct ErrorResponse {
    detail: String,
}
```

### External API Integration
- **Base URL**: `https://official-joke-api.appspot.com`
- **Endpoints Used**:
  - `/jokes/types` - Get available joke types
  - `/random_joke` - Get random joke
  - `/jokes/{type}/random` - Get random joke of specific type

## API Endpoints

### 1. Get Joke Types
**Endpoint**: `GET /jokes/types`

**Description**: Returns list of available joke categories

**Response**:
```json
["general", "programming", "knock-knock", "dad"]
```

**Caching**: 5 minutes (300 seconds)

### 2. Get Random Joke
**Endpoint**: `GET /jokes/random`

**Query Parameters**:
- `keyword` (optional): Filter jokes by keyword

**Examples**:
```bash
# Random joke from any category
curl http://127.0.0.1:8000/jokes/random

# Random joke with keyword filter
curl "http://127.0.0.1:8000/jokes/random?keyword=programming"
```

**Response**:
```json
{
  "id": 123,
  "type": "programming",
  "setup": "Why do programmers prefer dark mode?",
  "punchline": "Because the light attracts bugs."
}
```

### 3. Get Joke by Type
**Endpoint**: `GET /jokes/type/{type}`

**Path Parameters**:
- `type`: Joke category (e.g., "programming", "general")

**Query Parameters**:
- `keyword` (optional): Additional keyword filtering

**Example**:
```bash
curl http://127.0.0.1:8000/jokes/type/programming
```

## Caching System

### Redis Configuration
- **Host**: `127.0.0.1:6379`
- **Database**: Default (0)
- **TTL**: 300 seconds (5 minutes)

### Cache Keys
- `joke_types` - Cached joke type list
- `joke_random_{keyword}` - Random jokes with optional keyword
- `joke_type_{type}_{keyword}` - Type-specific jokes with optional keyword

### Cache Functions
```rust
async fn cache_response(redis_client: &RedisClient, key: &str, data: &Joke) -> anyhow::Result<()>
async fn get_cached_response(redis_client: &RedisClient, key: &str) -> Option<Joke>
```

## Fallback System

### Local Joke Dataset
```rust
static FALLBACK_JOKES: Lazy<Mutex<Vec<Joke>>> = Lazy::new(|| {
    Mutex::new(vec![
        Joke {
            id: 1,
            joke_type: "general".to_string(),
            setup: "Why did the tomato turn red?".to_string(),
            punchline: "Because it saw the salad dressing!".to_string(),
        },
        // Additional fallback jokes...
    ])
});
```

### Fallback Triggers
- External API unavailable
- Network connectivity issues
- HTTP request timeouts
- Invalid response format

## Request Processing Flow

### 1. Cache Check
- Generate cache key based on endpoint and parameters
- Check Redis for existing cached response
- Return cached data if available and not expired

### 2. External API Request
- Construct URL for Official Joke API
- Send HTTP GET request with reqwest
- Parse JSON response into Joke struct

### 3. Keyword Filtering
- Apply keyword filter to joke content
- Search setup, punchline, and type fields
- Case-insensitive matching

### 4. Response Caching
- Store successful responses in Redis
- Set TTL to 5 minutes
- Handle cache errors gracefully

### 5. Fallback Handling
- Use local dataset if external API fails
- Apply same filtering logic
- Ensure consistent