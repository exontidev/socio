use std::{net::IpAddr, path::PathBuf};

type Result<T> = core::result::Result<T, Error>;

pub struct RawSecret(pub Vec<u8>);
impl RawSecret {
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Clone)]
pub struct RawPublic(pub Vec<u8>);
impl RawPublic {
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub crypto: CryptoConfig,
    pub tokens: TokenDuration,
    pub network: NetworkConfig,
}

impl Config {
    pub async fn load(path: &str) -> Result<Self> {
        let contents = tokio::fs::read_to_string(path).await?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub async fn load_secret(&self) -> Result<RawSecret> {
        let content = tokio::fs::read(&self.crypto.private).await?;
        Ok(RawSecret(content))
    }

    pub async fn load_public(&self) -> Result<RawPublic> {
        let content = tokio::fs::read(&self.crypto.public).await?;
        Ok(RawPublic(content))
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CryptoConfig {
    pub private: PathBuf,
    pub public: PathBuf,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct NetworkConfig {
    pub ip: IpAddr,
    pub port: u16,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TokenDuration {
    #[serde(with = "humantime_serde")]
    pub access_token_duration: std::time::Duration,

    #[serde(with = "humantime_serde")]
    pub refresh_token_duration: std::time::Duration,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] tokio::io::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}
