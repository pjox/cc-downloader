#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    Reqwest(reqwest::Error),
    Tokio(tokio::io::Error),
    Url(url::ParseError),
    Indicatif(indicatif::style::TemplateError),
    Join(tokio::task::JoinError),
    Custom(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl From<tokio::io::Error> for Error {
    fn from(err: tokio::io::Error) -> Self {
        Error::Tokio(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::Url(err)
    }
}

impl From<indicatif::style::TemplateError> for Error {
    fn from(err: indicatif::style::TemplateError) -> Self {
        Error::Indicatif(err)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Error::Join(err)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::Custom(s)
    }
}
