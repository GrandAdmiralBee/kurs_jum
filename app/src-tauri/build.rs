use serde::{Deserialize, Serialize};
use sysinfo::{DiskExt, System, SystemExt};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub disk: Vec<String>,
    pub host_name: String,
}

impl Settings {
    pub fn ser(&self, path: &str) -> anyhow::Result<()> {
        let serialized = serde_json::ser::to_string_pretty(self)?;
        std::fs::write(path, serialized)?;
        Ok(())
    }

    pub fn deser(path: &str) -> anyhow::Result<Settings> {
        let deserialized: Settings = serde_json::de::from_reader(std::fs::File::open(path)?)?;
        Ok(deserialized)
    }
}

fn main() {
    let sys = System::new_all();

    let disk: Vec<String> = sys
        .disks()
        .iter()
        .map(|x| x.name().to_string_lossy().to_string())
        .collect();
    let name = sys.host_name().unwrap();

    let settings = Settings {
        disk: disk.to_vec(),
        host_name: name,
    };

    settings.ser("/home/karim/pc_settings.json").unwrap();

    tauri_build::build()
}
