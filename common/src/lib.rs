pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct User {
        pub id: u32,
        pub username: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Tweet {
        pub id: u32,
        pub user_id: u32,
        pub content: String,
    }
}
