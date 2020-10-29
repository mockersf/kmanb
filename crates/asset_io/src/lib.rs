use std::path::{Path, PathBuf};

use bevy::asset::AssetIo;

mod plugin;
pub use plugin::InMemoryAssetPlugin;

include!(concat!(env!("OUT_DIR"), "/include_all_assets.rs"));

pub struct InMemoryAssetIo {
    loaded: std::collections::HashMap<&'static Path, &'static [u8]>,
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

    pub fn add_entity(&mut self, path: &'static Path, data: &'static [u8]) {
        self.loaded.insert(path, data);
    }
}

impl AssetIo for InMemoryAssetIo {
    fn load_path<'a>(
        &'a self,
        path: &'a Path,
    ) -> bevy::utils::BoxedFuture<'a, Result<Vec<u8>, bevy::asset::AssetIoError>> {
        Box::pin(async move {
            self.loaded
                .get(path)
                .map(|b| b.to_vec())
                .ok_or_else(|| bevy::asset::AssetIoError::NotFound(path.to_path_buf()))
        })
    }

    fn read_directory(
        &self,
        _path: &Path,
    ) -> Result<Box<dyn Iterator<Item = PathBuf>>, bevy::asset::AssetIoError> {
        Ok(Box::new(std::iter::empty::<PathBuf>()))
    }

    fn is_directory(&self, _path: &Path) -> bool {
        false
    }

    fn watch_path_for_changes(&self, _path: &Path) -> Result<(), bevy::asset::AssetIoError> {
        Ok(())
    }

    fn watch_for_changes(&self) -> Result<(), bevy::asset::AssetIoError> {
        Ok(())
    }
}
