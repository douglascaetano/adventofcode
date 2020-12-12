#[derive(Debug)]
pub struct EntriesError {
    pub error: String,
}

impl<T> From<T> for EntriesError
where
    T: ToString,
{
    fn from(error: T) -> Self {
        Self {
            error: error.to_string(),
        }
    }
}
