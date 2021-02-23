pub enum CrawlError {
    ArgError,
    ParseError,
    FetchError,
}

pub type Result<T> = std::result::Result<T, CrawlError>;
