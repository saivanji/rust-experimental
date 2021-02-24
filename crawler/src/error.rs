pub enum CrawlError {
    WebsiteError(WebsiteErrorReason),
    DirectoryError(DirectoryErrorReason),
    FetchError,
    JoinError,
}

pub enum WebsiteErrorReason {
    InvalidScheme,
    ParseFailure,
    NotProvided,
}

pub enum DirectoryErrorReason {
    NotProvided,
}

pub type Result<T> = std::result::Result<T, CrawlError>;
