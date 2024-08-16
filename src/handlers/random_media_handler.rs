use axum::extract::State;
use reqwest::Response;
use serde::Deserialize;

use crate::utils::{
    response::{VoidsongError, VoidsongImage},
    state::{user_agent, AppState},
    url::{fetch_image, preflight_check},
};

#[derive(Deserialize)]
struct CatAsService {
    _id: String,
}

#[derive(Deserialize)]
struct TheCatAPI {
    url: String,
}

pub async fn cat(State(state): State<AppState>) -> Result<VoidsongImage, VoidsongError> {
    let urls: Vec<&str> = vec![
        "https://cataas.com/cat?json=true",
        "https://api.thecatapi.com/v1/images/search",
    ];

    // Check if the APIs are available
    let (success, url) = preflight_check(&state.client, urls).await;
    if !success {
        return Err(VoidsongError::ServiceUnavailable);
    }

    // Get the image URL
    let get_url: Response = match state
        .client
        .get(url.unwrap())
        .headers(user_agent())
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err(VoidsongError::FailedToFetchImage),
    };

    let url: String = match url.unwrap() {
        "https://cataas.com/cat?json=true" => match get_url.json::<CatAsService>().await {
            Ok(response) => format!("https://cataas.com/cat/{}", response._id),
            Err(_) => return Err(VoidsongError::FailedToFetchImage),
        },
        "https://api.thecatapi.com/v1/images/search" => {
            match get_url.json::<Vec<TheCatAPI>>().await {
                Ok(response) => response[0].url.clone(),
                Err(_) => return Err(VoidsongError::FailedToFetchImage),
            }
        }
        _ => return Err(VoidsongError::FailedToFetchImage),
    };

    let image: Option<VoidsongImage> = fetch_image(&state.client, url.as_str()).await;
    image.ok_or(VoidsongError::FailedToFetchImage)
}

/* -------------------------------------------------------------------------- */

#[derive(Deserialize)]
struct DogCEO {
    message: String,
}

pub async fn dog(State(state): State<AppState>) -> Result<VoidsongImage, VoidsongError> {
    let urls: Vec<&str> = vec!["https://dog.ceo/api/breeds/image/random"];

    // Check if the APIs are available
    let (success, url) = preflight_check(&state.client, urls).await;
    if !success {
        return Err(VoidsongError::ServiceUnavailable);
    }

    // Get the image URL
    let get_url: Response = match state
        .client
        .get(url.unwrap())
        .headers(user_agent())
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err(VoidsongError::FailedToFetchImage),
    };

    let url: String = match get_url.json::<DogCEO>().await {
        Ok(response) => response.message,
        Err(_) => return Err(VoidsongError::FailedToFetchImage),
    };

    let image: Option<VoidsongImage> = fetch_image(&state.client, url.as_str()).await;
    image.ok_or(VoidsongError::FailedToFetchImage)
}

/* -------------------------------------------------------------------------- */

#[derive(Deserialize)]
struct RandomFox {
    image: String,
}

pub async fn fox(State(state): State<AppState>) -> Result<VoidsongImage, VoidsongError> {
    let urls: Vec<&str> = vec!["https://randomfox.ca/floof"];

    // Check if the APIs are available
    let (success, url) = preflight_check(&state.client, urls).await;
    if !success {
        return Err(VoidsongError::ServiceUnavailable);
    }

    // Get the image URL
    let get_url: Response = match state
        .client
        .get(url.unwrap())
        .headers(user_agent())
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err(VoidsongError::FailedToFetchImage),
    };

    let url: String = match get_url.json::<RandomFox>().await {
        Ok(response) => response.image,
        Err(_) => return Err(VoidsongError::FailedToFetchImage),
    };

    let image: Option<VoidsongImage> = fetch_image(&state.client, url.as_str()).await;
    image.ok_or(VoidsongError::FailedToFetchImage)
}

/* -------------------------------------------------------------------------- */

#[derive(Deserialize)]
struct BunniesIo {
    pub media: Media,
}

#[derive(Deserialize)]
struct Media {
    pub poster: String,
}

pub async fn bunny(State(state): State<AppState>) -> Result<VoidsongImage, VoidsongError> {
    let urls: Vec<&str> = vec!["https://api.bunnies.io/v2/loop/random/?media=gif,png"];

    // Check if the APIs are available
    let (success, url) = preflight_check(&state.client, urls).await;
    if !success {
        return Err(VoidsongError::ServiceUnavailable);
    }

    // Get the image URL
    let get_url: Response = match state
        .client
        .get(url.unwrap())
        .headers(user_agent())
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err(VoidsongError::FailedToFetchImage),
    };

    let url: String = match get_url.json::<BunniesIo>().await {
        Ok(response) => response.media.poster,
        Err(_) => return Err(VoidsongError::FailedToFetchImage),
    };

    let image: Option<VoidsongImage> = fetch_image(&state.client, url.as_str()).await;
    image.ok_or(VoidsongError::FailedToFetchImage)
}

/* -------------------------------------------------------------------------- */

#[derive(Deserialize)]
struct RandomDuck {
    url: String,
}

pub async fn duck(State(state): State<AppState>) -> Result<VoidsongImage, VoidsongError> {
    let urls: Vec<&str> = vec!["https://random-d.uk/api/v1/random?type=png"];

    // Check if the APIs are available
    let (success, url) = preflight_check(&state.client, urls).await;
    if !success {
        return Err(VoidsongError::ServiceUnavailable);
    }

    // Get the image URL
    let get_url: Response = match state
        .client
        .get(url.unwrap())
        .headers(user_agent())
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err(VoidsongError::FailedToFetchImage),
    };

    let url: String = match get_url.json::<RandomDuck>().await {
        Ok(response) => response.url,
        Err(_) => return Err(VoidsongError::FailedToFetchImage),
    };

    let image: Option<VoidsongImage> = fetch_image(&state.client, url.as_str()).await;
    image.ok_or(VoidsongError::FailedToFetchImage)
}
