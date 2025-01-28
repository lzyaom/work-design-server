use serde::Deserialize;
pub mod document;
pub mod log;
pub mod program;
pub mod task;
pub mod user;

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
