#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GameServer {
    account: GameServerAccount,
    server: GameServerInformation
    // TODO: Investigate the analytics response
}

impl GameServer {
    pub fn get_account_info(&self) -> GameServerAccount {
        self.account.clone()
    }

    pub fn get_server_info(&self) -> (u32, String) {
        (self.server.id, self.server.name.clone())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameServerAccount {
    pub id: u32,
    pub domain: String,
    pub name: String,
    // currency
    pub online_mode: bool,
    pub game_type: GameType,
    pub log_events: bool // TODO: Figure out how this is useful and document
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GameServerInformation {
    id: u32,
    name: String
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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