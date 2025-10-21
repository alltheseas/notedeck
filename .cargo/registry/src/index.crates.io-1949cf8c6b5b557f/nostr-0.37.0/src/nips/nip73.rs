// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! NIP73: External Content IDs
//!
//! <https://github.com/nostr-protocol/nips/blob/master/73.md>

use alloc::string::{String, ToString};
use core::fmt;
use core::str::FromStr;

use crate::types::Url;

const HASHTAG: &str = "#";
const GEOHASH: &str = "geo:";
const BOOK: &str = "isbn:";
const PODCAST_FEED: &str = "podcast:guid:";
const PODCAST_EPISODE: &str = "podcast:item:guid:";
const PODCAST_PUBLISHER: &str = "podcast:publisher:guid:";
const MOVIE: &str = "isan:";
const PAPER: &str = "doi:";

/// NIP73 error
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid external content
    InvalidExternalContent,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidExternalContent => write!(f, "invalid external content ID"),
        }
    }
}

/// External Content ID
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExternalContentId {
    /// URL
    Url(Url),
    /// Hashtag
    Hashtag(String),
    /// Geohash
    Geohash(String),
    /// Book
    Book(String),
    /// Podcast Feed
    PodcastFeed(String),
    /// Podcast Episode
    PodcastEpisode(String),
    /// Podcast Publisher
    PodcastPublisher(String),
    /// Movie
    Movie(String),
    /// Paper
    Paper(String),
}

impl fmt::Display for ExternalContentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Url(url) => write!(f, "{url}"),
            Self::Hashtag(hashtag) => write!(f, "{HASHTAG}{hashtag}"),
            Self::Geohash(hash) => write!(f, "{GEOHASH}{hash}"),
            Self::Book(id) => write!(f, "{BOOK}{id}"),
            Self::PodcastFeed(guid) => write!(f, "{PODCAST_FEED}{guid}"),
            Self::PodcastEpisode(guid) => write!(f, "{PODCAST_EPISODE}{guid}"),
            Self::PodcastPublisher(guid) => write!(f, "{PODCAST_PUBLISHER}{guid}"),
            Self::Movie(movie) => write!(f, "{MOVIE}{movie}"),
            Self::Paper(paper) => write!(f, "{PAPER}{paper}"),
        }
    }
}

impl FromStr for ExternalContentId {
    type Err = Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = content.strip_prefix(HASHTAG) {
            return Ok(Self::Hashtag(stripped.to_string()));
        }

        if let Some(stripped) = content.strip_prefix(GEOHASH) {
            return Ok(Self::Geohash(stripped.to_string()));
        }

        if let Some(stripped) = content.strip_prefix(BOOK) {
            return Ok(Self::Book(stripped.to_string()));
        }

        if let Some(stripped) = content.strip_prefix(PODCAST_FEED) {
            return Ok(Self::PodcastFeed(stripped.to_string()));
        }

        if let Some(stripped) = content.strip_prefix(PODCAST_EPISODE) {
            return Ok(Self::PodcastEpisode(stripped.to_string()));
        }

        if let Some(stripped) = content.strip_prefix(PODCAST_PUBLISHER) {
            return Ok(Self::PodcastPublisher(stripped.to_string()));
        }

        if let Some(stripped) = content.strip_prefix(MOVIE) {
            return Ok(Self::Movie(stripped.to_string()));
        }

        if let Some(stripped) = content.strip_prefix(PAPER) {
            return Ok(Self::Paper(stripped.to_string()));
        }

        if let Ok(url) = Url::parse(content) {
            return Ok(Self::Url(url));
        }

        Err(Error::InvalidExternalContent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        assert_eq!(
            ExternalContentId::Url("https://example.com".parse().unwrap()).to_string(),
            "https://example.com/"
        );
        assert_eq!(
            ExternalContentId::Hashtag("rust".to_string()).to_string(),
            "#rust"
        );
        assert_eq!(
            ExternalContentId::Geohash("u4pruydqqvj".to_string()).to_string(),
            "geo:u4pruydqqvj"
        );
        assert_eq!(
            ExternalContentId::Book("978-3-16-148410-0".to_string()).to_string(),
            "isbn:978-3-16-148410-0"
        );
        assert_eq!(
            ExternalContentId::PodcastFeed("feed-guid".to_string()).to_string(),
            "podcast:guid:feed-guid"
        );
        assert_eq!(
            ExternalContentId::PodcastEpisode("episode-guid".to_string()).to_string(),
            "podcast:item:guid:episode-guid"
        );
        assert_eq!(
            ExternalContentId::PodcastPublisher("publisher-guid".to_string()).to_string(),
            "podcast:publisher:guid:publisher-guid"
        );
        assert_eq!(
            ExternalContentId::Movie("movie-id".to_string()).to_string(),
            "isan:movie-id"
        );
        assert_eq!(
            ExternalContentId::Paper("10.1000/182".to_string()).to_string(),
            "doi:10.1000/182"
        );
    }

    #[test]
    fn test_parsing() {
        assert_eq!(
            ExternalContentId::from_str("https://example.com").unwrap(),
            ExternalContentId::Url(Url::parse("https://example.com").unwrap())
        );
        assert_eq!(
            ExternalContentId::from_str("#rust").unwrap(),
            ExternalContentId::Hashtag("rust".to_string())
        );
        assert_eq!(
            ExternalContentId::from_str("geo:u4pruydqqvj").unwrap(),
            ExternalContentId::Geohash("u4pruydqqvj".to_string())
        );
        assert_eq!(
            ExternalContentId::from_str("isbn:978-3-16-148410-0").unwrap(),
            ExternalContentId::Book("978-3-16-148410-0".to_string())
        );
        assert_eq!(
            ExternalContentId::from_str("podcast:guid:feed-guid").unwrap(),
            ExternalContentId::PodcastFeed("feed-guid".to_string())
        );
        assert_eq!(
            ExternalContentId::from_str("podcast:item:guid:episode-guid").unwrap(),
            ExternalContentId::PodcastEpisode("episode-guid".to_string())
        );
        assert_eq!(
            ExternalContentId::from_str("podcast:publisher:guid:publisher-guid").unwrap(),
            ExternalContentId::PodcastPublisher("publisher-guid".to_string())
        );
        assert_eq!(
            ExternalContentId::from_str("isan:movie-id").unwrap(),
            ExternalContentId::Movie("movie-id".to_string())
        );
        assert_eq!(
            ExternalContentId::from_str("doi:10.1000/182").unwrap(),
            ExternalContentId::Paper("10.1000/182".to_string())
        );
    }

    #[test]
    fn test_invalid_content() {
        assert_eq!(
            ExternalContentId::from_str("hello"),
            Err(Error::InvalidExternalContent)
        );
    }
}
