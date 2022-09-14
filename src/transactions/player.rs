#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Player {
    id: u32,
    name: String,
    uuid: String
}

impl Player {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn uuid(&self) -> &str {
        self.uuid.as_str()
    }
}