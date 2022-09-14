pub enum Endpoint {
    Information,
    DueTransactions,
    OnlineCommands(u32)
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Endpoint::Information => "https://plugin.tebex.io/information".into(),
            Endpoint::DueTransactions => "https://plugin.tebex.io/queue".into(),
            Endpoint::OnlineCommands(id) => format!("https://plugin.tebex.io/queue/online-commands/{}", id)
        }
    }
}