pub mod config {
    struct Config {
        root_path: String,
    }

    impl Config {
        pub fn new() -> Config {
            Config { 
                root_path: "root/".to_string()
            }
        }

        pub fn set_root_path(&mut self, path: String) {
            self.root_path = path;
        }
    }

    
}