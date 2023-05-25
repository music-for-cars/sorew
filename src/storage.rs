use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub username: String,
    pub latest_track_uri: String,
    pub latest_track_title: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Store {
    pub entries: Vec<Entry>,
}
