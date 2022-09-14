#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Transaction {
    id: u32,
    command: String,
    payment: u32,
    package: u32,
    conditions: Conditions
}

impl Transaction {
    pub fn command(&self) -> String {
        self.command.clone()
    }

    pub fn conditions(&self) -> Conditions {
        self.conditions
    } 

    pub fn delete(&self) {}
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct Conditions {
    #[serde(rename(serialize = "delay", deserialize = "delay"))]
    pub delay: u32,
    #[serde(rename(serialize = "slots", deserialize = "slots"))]
    pub slots: u32
}