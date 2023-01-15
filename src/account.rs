use reqwest::{
    blocking::{Client, Response},
    StatusCode,
};
use thiserror::Error;

use crate::{
    attributes::MinecraftPlayerAttributes, blocklist::MinecraftBlocklist, profile::MinecraftProfile,
};

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

    fn api_get(&self, path: &str) -> Result<Response, MinecraftServiceError> {
        let response = self
            .client
            .get(format!("https://api.minecraftservices.com/{}", path))
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()?;

        match response.status() {
            StatusCode::OK => {
                return Ok(response);
            }

            StatusCode::UNAUTHORIZED => {
                return Err(MinecraftServiceError::InvalidAccessToken);
            }

            _ => {
                return Err(MinecraftServiceError::UnknownError);
            }
        }
    }

    pub fn get_attributes(&self) -> Result<MinecraftPlayerAttributes, MinecraftServiceError> {
        let response = self.api_get("player/attributes")?;
        let attributes: MinecraftPlayerAttributes = serde_json::from_reader(response)?;
        return Ok(attributes);
    }

    pub fn get_blocklist(&self) -> Result<MinecraftBlocklist, MinecraftServiceError> {
        let response = self.api_get("privacy/blocklist")?;
        let blocklist: MinecraftBlocklist = serde_json::from_reader(response)?;
        return Ok(blocklist);
    }

    pub fn get_profile(&self) -> Result<MinecraftProfile, MinecraftServiceError> {
        let response = self.api_get("minecraft/profile")?;
        let profile: MinecraftProfile = serde_json::from_reader(response)?;
        return Ok(profile);
    }
}
