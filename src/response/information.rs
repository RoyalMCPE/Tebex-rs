#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GameServerInformationResponse {
    pub account: GameServerAccount,
    pub server: GameServerInformation
    // TODO: Investigate the analytics response
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GameServerAccount {
    pub id: u32,
    pub domain: String,
    pub name: String,
    // currency
    pub online_mode: bool,
    pub game_type: GameType,
    pub log_events: bool
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GameServerInformation {
    id: u32,
    name: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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