pub enum CrawlError {
    ArgError,
    ParseError,
    FetchError,
    JoinError,
}

pub type Result<T> = std::result::Result<T, CrawlError>;
