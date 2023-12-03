#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error from custom server'{0}'")]
    CustomApiError(&'static str),
}
