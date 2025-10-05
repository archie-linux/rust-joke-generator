use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use redis::{Commands, Client as RedisClient};
use once_cell::sync::Lazy;
use std::sync::Mutex;

// Base URL for Official Joke API
const JOKE_API: &str = "https://official-joke-api.appspot.com";

// Joke response model
#[derive(Serialize, Deserialize, Clone)]
struct Joke {
    id: i32,
    #[serde(rename = "type")]
    joke_type: String,
    setup: String,
    punchline: String,
}

// Error response model
#[derive(Serialize)]
struct ErrorResponse {
    detail: String,
}

// Custom dataset (fallback)
static FALLBACK_JOKES: Lazy<Mutex<Vec<Joke>>> = Lazy::new(|| {
    Mutex::new(vec![
        Joke {
            id: 1,
            joke_type: "general".to_string(),
            setup: "Why did the tomato turn red?".to_string(),
            punchline: "Because it saw the salad dressing!".to_string(),
        },
        Joke {
            id: 2,
            joke_type: "programming".to_string(),
            setup: "Why do programmers prefer dark mode?".to_string(),
            punchline: "Because the light attracts bugs.".to_string(),
        },
        Joke {
            id: 3,
            joke_type: "knock-knock".to_string(),
            setup: "Knock knock. Who's there?".to_string(),
            punchline: "Boo. Boo who? Don't cry, it's just a joke!".to_string(),
        },
    ])
});

// Cache helper functions
async fn cache_response(redis_client: &RedisClient, key: &str, data: &Joke) -> anyhow::Result<()> {
    let mut conn = redis_client.get_connection()?;
    let serialized = serde_json::to_string(data).map_err(|e| anyhow::anyhow!("Serialization error: {}", e))?;
    conn.set_ex::<_, _, ()>(key, serialized, 300)?;
    Ok(())
}

async fn get_cached_response(redis_client: &RedisClient, key: &str) -> Option<Joke> {
    let mut conn = redis_client.get_connection().ok()?;
    let cached: Option<String> = conn.get(key).ok()?;
    cached.and_then(|data| serde_json::from_str(&data).ok())
}

// Get list of joke types
#[get("/jokes/types")]
async fn get_types(redis_client: web::Data<RedisClient>) -> impl Responder {
    let cache_key = "joke_types";
    if let Some(cached) = get_cached_response(&redis_client, cache_key).await {
        return HttpResponse::Ok().json(cached);
    }

    let client = Client::new();
    match client.get(format!("{}/jokes/types", JOKE_API)).send().await {
        Ok(response) if response.status().is_success() => {
            let types: Vec<String> = response.json().await.unwrap_or_else(|_| vec![]);
            let serialized = match serde_json::to_string(&types) {
                Ok(s) => s,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        detail: format!("Serialization error: {}", e),
                    });
                }
            };
            let _ = redis_client
                .get_connection()
                .and_then(|mut conn| conn.set_ex::<_, _, ()>(cache_key, serialized, 300));
            HttpResponse::Ok().json(types)
        }
        _ => {
            let fallback_types = vec![
                "general".to_string(),
                "programming".to_string(),
                "knock-knock".to_string(),
            ];
            let serialized = match serde_json::to_string(&fallback_types) {
                Ok(s) => s,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        detail: format!("Serialization error: {}", e),
                    });
                }
            };
            let _ = redis_client
                .get_connection()
                .and_then(|mut conn| conn.set_ex::<_, _, ()>(cache_key, serialized, 300));
            HttpResponse::Ok().json(fallback_types)
        }
    }
}

// Get a random joke
#[get("/jokes/random")]
async fn get_random_joke(
    query: web::Query<std::collections::HashMap<String, String>>,
    redis_client: web::Data<RedisClient>,
) -> impl Responder {
    let keyword = query.get("keyword").map(|s| s.to_lowercase());
    let cache_key = format!("joke_random_{}", keyword.as_deref().unwrap_or("no_keyword"));

    if let Some(cached) = get_cached_response(&redis_client, &cache_key).await {
        return HttpResponse::Ok().json(cached);
    }

    let client = Client::new();
    // Check if keyword matches a joke type
    let url = if let Some(kw) = &keyword {
        let types = match client.get(format!("{}/jokes/types", JOKE_API)).send().await {
            Ok(response) if response.status().is_success() => response.json().await.unwrap_or_else(|_| vec![]),
            _ => vec!["general".to_string(), "programming".to_string(), "knock-knock".to_string()],
        };
        if types.contains(&kw.to_string()) {
            format!("{}/jokes/{}/random", JOKE_API, kw)
        } else {
            format!("{}/random_joke", JOKE_API)
        }
    } else {
        format!("{}/random_joke", JOKE_API)
    };

    match client.get(&url).send().await {
        Ok(response) if response.status().is_success() => {
            let joke: Joke = if url.contains("/random") {
                let jokes: Vec<Joke> = response.json().await.unwrap_or_else(|_| vec![]);
                jokes.get(0).cloned().unwrap_or_else(|| FALLBACK_JOKES.lock().unwrap()[0].clone())
            } else {
                response.json().await.unwrap_or_else(|_| FALLBACK_JOKES.lock().unwrap()[0].clone())
            };
            if let Some(kw) = &keyword {
                if !format!("{} {} {}", joke.joke_type, joke.setup, joke.punchline)
                    .to_lowercase()
                    .contains(kw)
                {
                    return HttpResponse::NotFound().json(ErrorResponse {
                        detail: "No joke found with keyword".to_string(),
                    });
                }
            }
            let _ = cache_response(&redis_client, &cache_key, &joke).await;
            HttpResponse::Ok().json(joke)
        }
        _ => {
            for joke in FALLBACK_JOKES.lock().unwrap().iter() {
                if keyword
                    .as_ref()
                    .map(|kw| format!("{} {} {}", joke.joke_type, joke.setup, joke.punchline).to_lowercase().contains(kw))
                    .unwrap_or(true)
                {
                    let _ = cache_response(&redis_client, &cache_key, joke).await;
                    return HttpResponse::Ok().json(joke);
                }
            }
            HttpResponse::NotFound().json(ErrorResponse {
                detail: "No joke found".to_string(),
            })
        }
    }
}

// Get a joke by type
#[get("/jokes/type/{type}")]
async fn get_joke_by_type(
    path: web::Path<String>,
    query: web::Query<std::collections::HashMap<String, String>>,
    redis_client: web::Data<RedisClient>,
) -> impl Responder {
    let joke_type = path.into_inner();
    let keyword = query.get("keyword").map(|s| s.to_lowercase());
    let cache_key = format!("joke_type_{}_{}", joke_type, keyword.as_deref().unwrap_or("no_keyword"));

    if let Some(cached) = get_cached_response(&redis_client, &cache_key).await {
        return HttpResponse::Ok().json(cached);
    }

    let client = Client::new();
    match client
        .get(format!("{}/jokes/{}/random", JOKE_API, joke_type))
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            let jokes: Vec<Joke> = response.json().await.unwrap_or_else(|_| vec![]);
            if let Some(joke) = jokes.get(0) {
                if let Some(kw) = &keyword {
                    if !format!("{} {} {}", joke.joke_type, joke.setup, joke.punchline)
                        .to_lowercase()
                        .contains(kw)
                    {
                        return HttpResponse::NotFound().json(ErrorResponse {
                            detail: "No joke found with keyword".to_string(),
                        });
                    }
                }
                let _ = cache_response(&redis_client, &cache_key, joke).await;
                HttpResponse::Ok().json(joke)
            } else {
                HttpResponse::NotFound().json(ErrorResponse {
                    detail: "Joke type not found".to_string(),
                })
            }
        }
        _ => {
            for joke in FALLBACK_JOKES.lock().unwrap().iter() {
                if joke.joke_type == joke_type
                    && keyword
                        .as_ref()
                        .map(|kw| format!("{} {} {}", joke.joke_type, joke.setup, joke.punchline).to_lowercase().contains(kw))
                        .unwrap_or(true)
                {
                    let _ = cache_response(&redis_client, &cache_key, joke).await;
                    return HttpResponse::Ok().json(joke);
                }
            }
            HttpResponse::NotFound().json(ErrorResponse {
                detail: "Joke type or joke not found".to_string(),
            })
        }
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let redis_client = RedisClient::open("redis://127.0.0.1:6379/")
        .map_err(|e| anyhow::anyhow!("Failed to connect to Redis: {}", e))?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_client.clone()))
            .service(get_types)
            .service(get_random_joke)
            .service(get_joke_by_type)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;
    Ok(())
}
