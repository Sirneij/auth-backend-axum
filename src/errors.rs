#[derive(Debug)]
pub enum BackendError {
    DatabaseQueryError(String),
    UserExistsError(String),
    DataNotFound(String),
    WrongPassword,
    ArgonLibraryError(argon2::Error),
}

impl axum::response::IntoResponse for BackendError {
    #[tracing::instrument]
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::DatabaseQueryError(ref e) => {
                tracing::event!(tracing::Level::ERROR, "Database Query error: {}", e);

                (
                    axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                    format!("Database Query error: {}", e),
                )
            }

            Self::UserExistsError(ref e) => {
                tracing::event!(tracing::Level::ERROR, "User: {} already exists", e);
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    format!("User with the email address, {}, already exists.", e),
                )
            }
            Self::DataNotFound(ref e) => {
                tracing::event!(tracing::Level::ERROR, "Queried data not found");
                (axum::http::StatusCode::NOT_FOUND, e.to_string())
            }

            Self::WrongPassword => (
                axum::http::StatusCode::UNAUTHORIZED,
                "Wrong e-mail/password combination".to_string(),
            ),
            Self::ArgonLibraryError(ref e) => (
                axum::http::StatusCode::UNAUTHORIZED,
                format!("Cannot verifiy password: {:?}", e),
            ),
        };
        (
            status,
            axum::Json(serde_json::json!({ "error": error_message })),
        )
            .into_response()
    }
}
