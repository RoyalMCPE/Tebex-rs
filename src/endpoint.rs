pub enum Endpoint {
    Information,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Endpoint::Information => "https://plugin.tebex.io/information".into()
        }
    }
}