mod config;

use std::sync::LazyLock;
use self::config::Config;

pub static APP_CONFIG: LazyLock<Config> = LazyLock::new(|| {
    Config::load()
});