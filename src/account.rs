use reqwest::{
    blocking::{Client, Response},
    StatusCode,
};
use thiserror::Error;

use crate::profile::MinecraftProfile;

pub struct MinecraftAccount {
    access_token: String,
    client: Client,
}

#[derive(Error, Debug)]
pub enum MinecraftServiceError {
    #[error("The access token is invalid or was expired.")]
    InvalidAccessToken,

    #[error("An unexpected error has ocurred.")]
    UnknownError,

    #[error("{0}")]
    Request(#[from] reqwest::Error),

    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

impl MinecraftAccount {
    pub fn new(access_token: &str) -> Self {
        Self {
            access_token: access_token.to_string(),
            client: Client::new(),
        }
    }

    fn api_get(&self, path: &str) -> Result<Response, reqwest::Error> {
        let response = self
            .client
            .get(format!("https://api.minecraftservices.com/{}", path))
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send();
        return response;
    }

    pub fn get_profile(&self) -> Result<MinecraftProfile, MinecraftServiceError> {
        let response = self.api_get("minecraft/profile")?;

        match response.status() {
            StatusCode::OK => {
                let profile: MinecraftProfile = serde_json::from_reader(response)?;
                return Ok(profile);
            }

            StatusCode::UNAUTHORIZED => {
                return Err(MinecraftServiceError::InvalidAccessToken);
            }

            _ => {
                return Err(MinecraftServiceError::UnknownError);
            }
        }
    }
}
