use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref APP_CONFIG: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}
