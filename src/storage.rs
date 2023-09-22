use serde::{Deserialize, Serialize};

use crate::error::Error;

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

pub fn load_data<T>(path: &str) -> Result<T, Error>
where
    for<'de> T: Deserialize<'de>,
{
    serde_json::from_str(
        &std::fs::read_to_string(path).map_err(|e| Error::LoadDataError(e.to_string()))?,
    )
    .map_err(|e| Error::DeserializeDataError(e.to_string()))
}

pub fn save_data<T, S>(path: S, data: T) -> Result<(), Error>
where
    T: Serialize,
    S: AsRef<str>,
{
    let path = path.as_ref();
    std::fs::write(
        path,
        &serde_json::to_string(&data).map_err(|e| Error::SerializeDataError(e.to_string()))?,
    )
    .map_err(|e| Error::SaveDataError(e.to_string()))
}
