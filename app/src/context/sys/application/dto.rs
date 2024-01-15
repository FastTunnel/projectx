pub mod command {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct InitSystemCommand {
        pub config: String,
        pub value: String,
    }
}
