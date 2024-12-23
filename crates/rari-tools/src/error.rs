use std::borrow::Cow;

use rari_doc::error::{DocError, UrlError};
use rari_types::error::EnvError;
use rari_types::locale::LocaleError;
use rari_utils::error::RariIoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("Invalid slug: {0}")]
    InvalidSlug(Cow<'static, str>),
    #[error("Invalid url: {0}")]
    InvalidUrl(Cow<'static, str>),
    #[error("Invalid locale: {0}")]
    InvalidLocale(Cow<'static, str>),
    #[error("Orphaned doc exists: {0}")]
    OrphanedDocExists(Cow<'static, str>),
    #[error("Git error: {0}")]
    GitError(String),

    #[error(transparent)]
    LocaleError(#[from] LocaleError),
    #[error(transparent)]
    DocError(#[from] DocError),
    #[error(transparent)]
    EnvError(#[from] EnvError),
    #[error(transparent)]
    UrlError(#[from] UrlError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    RariIoError(#[from] RariIoError),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    YamlError(#[from] yaml_parser::SyntaxError),
    #[error("Invalid Redirection: {0}")]
    InvalidRedirectionEntry(String),
    #[error("Error reading redirects file: {0}")]
    ReadRedirectsError(String),
    #[error("Error writing redirects file: {0}")]
    WriteRedirectsError(String),
    #[error("Invalid 'from' URL for redirect: {0}")]
    InvalidRedirectFromURL(String),
    #[error("Invalid 'to' URL for redirect: {0}")]
    InvalidRedirectToURL(String),
    #[error(transparent)]
    RedirectError(#[from] RedirectError),
    #[error("Invalid yaml {0}")]
    InvalidFrontmatter(#[from] serde_yaml_ng::Error),
    #[error("Page has subpages: {0}")]
    HasSubpagesError(Cow<'static, str>),

    #[error("Unknonwn error")]
    Unknown(&'static str),
}

#[derive(Debug, Clone, Error)]
pub enum RedirectError {
    #[error("RedirectError: {0}")]
    Cycle(String),
    #[error("No cased version {0}")]
    NoCased(String),
}
