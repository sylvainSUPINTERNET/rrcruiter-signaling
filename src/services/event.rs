#[derive(serde::Deserialize, serde::Serialize)]
pub struct Event {
    pub topic: String,
    pub message: String,
    pub user_uuid: Option<String>
}
