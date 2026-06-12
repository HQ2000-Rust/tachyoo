//fatal server error
    #[derive(snafu::Snafu)]
pub enum ServerError {
    #[snafu(display("Failed to create a tokio runtime"))]
    RuntimeCreation {
        source: tokio::io:Error,
    },
}