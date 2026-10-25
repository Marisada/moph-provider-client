#[derive(Debug)]
pub enum Error {
    Custom(String),
    Base64Decode(base64::DecodeError),
    ClientBuilder(String),
    Json(serde_json::Error),
    Jwt(jsonwebtoken::errors::Error),
    Reqwest(reqwest::Error),
}

impl Error {
    pub(crate) fn new_custom(err: &str) -> Self {
        Self::Custom(err.to_owned())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom(s) => write!(f, "{s}"),
            Self::Base64Decode(e) => write!(f, "Base64Decode: {}", e),
            Self::ClientBuilder(s) => write!(f, "ClientBuilder: {}", s),
            Self::Json(e) => write!(f, "Json: {}", e),
            Self::Jwt(e) => write!(f, "Token: {}", e),
            Self::Reqwest(e) => write!(f, "Client: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<base64::DecodeError> for Error {
    fn from(error: base64::DecodeError) -> Self {
        Error::Base64Decode(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Error::Jwt(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error)
    }
}
