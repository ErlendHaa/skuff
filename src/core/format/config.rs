use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub enum StreamOrder {
    LastUsed,
    Lexographic,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub stream_order: Option<StreamOrder>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            stream_order: Some(StreamOrder::LastUsed),
        }
    }
}
impl Config {
    pub fn coalesce(
        explicit: Option<Config>,
        local: Option<Config>,
        global: Option<Config>,
    ) -> Config {
        Self::default()
            .rcoalesce(global)
            .rcoalesce(local)
            .rcoalesce(explicit)
    }

    fn rcoalesce(self, rhs: Option<Self>) -> Self {
        let rhs = match rhs {
            Some(config) => config,
            None => return self,
        };

        Self {
            stream_order: rhs.stream_order.or(self.stream_order),
        }
    }
}
