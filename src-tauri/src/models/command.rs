use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Ok,
    Error,
}

#[derive(Deserialize, Debug)]
pub struct CommandResult<T> {
    pub status: Status,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}
