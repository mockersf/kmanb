use bevy::asset::AssetIo;

include!(concat!(env!("OUT_DIR"), "/include_all_assets.rs"));

pub struct InMemoryAssetIo {
    loaded: std::collections::HashMap<&'static str, &'static [u8]>,
}

impl InMemoryAssetIo {
    pub fn new() -> Self {
        InMemoryAssetIo {
            loaded: std::collections::HashMap::new(),
        }
    }

    pub fn preloaded() -> Self {
        let mut new = InMemoryAssetIo {
            loaded: std::collections::HashMap::new(),
        };
        include_all_assets(&mut new);
        new
    }

    pub fn add_entity(&mut self, path: &'static str, data: &'static [u8]) {
        self.loaded.insert(path, data);
    }
}

impl AssetIo for InMemoryAssetIo {
    fn load_path<'a>(
        &'a self,
        path: &'a str,
    ) -> bevy::utils::BoxedFuture<'a, Result<Vec<u8>, bevy::asset::AssetIoError>> {
        Box::pin(async move {
            self.loaded.get(path).map(|b| b.to_vec()).ok_or_else(|| {
                bevy::asset::AssetIoError::NotFound(format!("asset {} was not preloaded", path))
            })
        })
    }

    fn read_directory(
        &self,
        _path: &str,
    ) -> Result<Box<dyn Iterator<Item = String>>, bevy::asset::AssetIoError> {
        Ok(Box::new(std::iter::empty::<String>()))
    }

    fn is_directory(&self, _path: &str) -> bool {
        false
    }

    fn watch_path_for_changes(&self, _path: &str) -> Result<(), bevy::asset::AssetIoError> {
        Ok(())
    }

    fn watch_for_changes(&self) -> Result<(), bevy::asset::AssetIoError> {
        Ok(())
    }

    fn extension<'a>(&self, path: &'a str) -> Option<&'a str> {
        std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
    }

    fn parent<'a>(&self, _path: &'a str) -> Option<&'a str> {
        Some("/")
    }

    fn sibling(&self, _path: &str, sibling: &str) -> Option<String> {
        Some(format!("/{}", sibling))
    }
}
