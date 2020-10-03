use std::path::Path;

use hocon::HoconLoader;
use serde::{Deserialize, Serialize};

pub fn read_from<'a, T>(path: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: Default + Serialize + Deserialize<'a>,
{
    let default = T::default();

    if Path::new(path).exists() {
        let default_as_json = serde_json::to_string(&default)?;
        let parsed: T = HoconLoader::new()
            .load_str(&default_as_json)?
            .load_file(path)?
            .resolve()?;
        Ok(parsed)
    } else {
        Ok(default)
    }
}
