use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthRequestData {
    pub input: i32,
}
