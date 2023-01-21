pub struct Config {
    pub port: u32,
    pub root_path: String,
    pub index: String,
    pub username: String,
    pub password: String
}

impl Config {
    pub fn new() -> Config {
        Config {
            port: 8080,
            root_path: "root/".to_string(),
            index: "index.html".to_string(),
            username: "user".to_string(),
            password: "password".to_string()
        }
    }

    pub fn set_index(&mut self, index: &str) {
        self.index = index.to_string();
    }
}