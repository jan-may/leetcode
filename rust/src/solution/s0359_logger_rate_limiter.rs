struct Logger {
    map: std::collections::HashMap<String, i32>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            map: std::collections::HashMap::new(),
        }
    }

    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if let Some(last_timestamp) = self.map.get(&message) {
            if timestamp - last_timestamp < 10 {
                return false;
            }
        }
        self.map.insert(message, timestamp);
        true
    }
}
