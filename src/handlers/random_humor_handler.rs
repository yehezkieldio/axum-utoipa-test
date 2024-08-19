use axum::extract::State;
use reqwest::Response;
use serde::Deserialize;

use crate::utils::{
    response::{VoidsongError, VoidsongHumor},
    state::{user_agent, AppState},
    url::preflight_check,
};

#[derive(Deserialize)]
struct ChuckNorrisFact {
    value: String,
}

pub async fn chuck_norris(State(state): State<AppState>) -> Result<VoidsongHumor, VoidsongError> {
    let urls: Vec<&str> = vec!["https://api.chucknorris.io/jokes/random"];

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
        Err(_) => return Err(VoidsongError::FailedToFetchContent),
    };

    let joke: String = match get_url.json::<ChuckNorrisFact>().await {
        Ok(response) => response.value,
        Err(_) => return Err(VoidsongError::FailedToFetchContent),
    };

    Ok(VoidsongHumor { joke })
}

/* -------------------------------------------------------------------------- */

#[derive(Deserialize)]
struct ICanHazDadJoke {
    joke: String,
}

pub async fn dad_joke(State(state): State<AppState>) -> Result<VoidsongHumor, VoidsongError> {
    let urls: Vec<&str> = vec!["https://icanhazdadjoke.com"];

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
        Err(_) => return Err(VoidsongError::FailedToFetchContent),
    };

    let joke: String = match get_url.json::<ICanHazDadJoke>().await {
        Ok(response) => response.joke,
        Err(_) => return Err(VoidsongError::FailedToFetchContent),
    };

    Ok(VoidsongHumor { joke })
}
