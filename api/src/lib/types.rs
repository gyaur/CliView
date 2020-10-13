pub type Error = Box<dyn std::error::Error>;
pub type GenericResult<T> = Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Event {
    Skip,
    VolumeUp,
}
