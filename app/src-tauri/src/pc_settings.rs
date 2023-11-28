use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub disk: Vec<String>,
    pub host_name: String,
}

impl Settings {
    pub fn ser(path: &str, settings: &Settings) -> anyhow::Result<()> {
        let serialized = serde_json::ser::to_string_pretty(settings)?;
        std::fs::write(path, serialized)?;
        Ok(())
    }

    pub fn deser(path: &str) -> anyhow::Result<Settings> {
        let deserialized: Settings = serde_json::de::from_reader(std::fs::File::open(path)?)?;
        Ok(deserialized)
    }
}
