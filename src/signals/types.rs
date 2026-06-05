/// A raw signal sighting from any scanner before resolution.
#[derive(Debug, Clone)]
pub struct RawSignal {
    pub scanner: String,
    pub id_type: String,
    pub id_value: String,
    pub rssi: Option<i32>,
    pub metadata: Option<String>,
}
