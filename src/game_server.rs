#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GameServer {
    account: GameServerAccount,
    server: GameServerInformation
    // TODO: Investigate the analytics response
}

impl GameServer {
    pub fn get_account(&self) -> GameServerAccount {
        self.account.clone()
    }

    pub fn get_information(&self) -> (u32, String) {
        (self.server.id, self.server.name.clone())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GameServerAccount {
    pub id: u32,
    pub domain: String,
    pub name: String,
    // currency
    pub online_mode: bool,
    pub game_type: GameType,
    pub log_events: bool // TODO: Provide documentation for what this is for?
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GameServerInformation {
    id: u32,
    name: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum GameType {
    #[serde(rename(serialize = "Minecraft: Java Edition", deserialize = "Minecraft: Java Edition"))]
    MinecraftJavaEdition,
    #[serde(rename(serialize = "Minecraft (Bedrock)", deserialize = "Minecraft (Bedrock)"))]
    MinecraftBedrockEdition,
    SevenDaysToDie,
    Ark,
    Atlas,
    CSGO,
    FiveM,
    GarysMod,
    HurtWorld,
    Rust,
    SpaceEngineers,
    TeamFortressTWO,
    Unturned
}