mod identity;
mod job;
mod remote;
mod source_type;

pub use identity::Identity;
pub use job::Job;
pub use remote::Remote;
pub use source_type::SourceType;

pub trait Source
where
    Self: Iterator<Item = Vec<Job>>,
    Self: Sync + Send,
    Self: Clone,
{
    fn kind(&self) -> SourceType;
}
