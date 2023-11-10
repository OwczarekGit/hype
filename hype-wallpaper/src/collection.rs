use rand::seq::IteratorRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::{
    collections::{HashMap, HashSet},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Collection {
    collections: HashMap<String, HashSet<PathBuf>>,
}

impl Collection {
    pub fn set_wallpaper(
        &self,
        collection: &str,
        wallpaper: &Path,
    ) -> Result<PathBuf, CollectionError> {
        let col = self
            .collections
            .get(collection)
            .ok_or(CollectionError::CollectionNotFound)?;

        col.iter()
            .find(|p| return p.file_name() == Some(wallpaper.as_os_str()))
            .cloned()
            .ok_or(CollectionError::ItemInCollectionNotFound)
    }

    pub fn random_from_collection(&self, collection: &str) -> Result<PathBuf, CollectionError> {
        let col = self
            .collections
            .get(collection)
            .ok_or(CollectionError::CollectionNotFound)?;

        let mut rng = thread_rng();
        Ok(col
            .iter()
            .choose(&mut rng)
            .ok_or(CollectionError::ItemInCollectionNotFound)?
            .clone())
    }
}

impl Collection {
    pub fn create_collection(&mut self, name: &str) {
        match self.collections.get(name) {
            Some(_) => {}
            None => {
                self.collections.insert(name.to_owned(), HashSet::default());
            }
        };
    }

    pub fn add_to_collection(&mut self, name: &str, paths: Vec<PathBuf>) {
        match self.collections.entry(name.to_string()) {
            Entry::Occupied(vec) => {
                vec.into_mut().extend(
                    paths
                        .iter()
                        .filter_map(|p| p.canonicalize().ok())
                        .collect::<Vec<_>>(),
                );
            }
            Entry::Vacant(entry) => {
                entry.insert(HashSet::from_iter(
                    paths
                        .iter()
                        .filter_map(|p| p.canonicalize().ok())
                        .collect::<Vec<_>>(),
                ));
            }
        };
    }

    pub fn get_collections(&self) -> Vec<String> {
        self.collections
            .keys()
            .map(|name| name.to_owned())
            .collect()
    }

    pub fn list_items_in_collection(&self, name: &str) -> Option<Vec<String>> {
        let collection = self.collections.get(name)?;
        Some(
            collection
                .iter()
                .filter_map(|el| {
                    el.file_name()
                        .and_then(|n| n.to_str().map(|s| s.to_owned()))
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl Collection {
    pub fn from_file(path: &Path) -> Result<Self, CollectionError> {
        let content = std::fs::read_to_string(path)?;
        toml::from_str(&content).map_err(|_| CollectionError::InvalidFormat)
    }

    pub fn save(self, path: &Path) -> Result<(), CollectionError> {
        let content = toml::to_string_pretty(&self)?;
        let mut file = std::fs::File::create(path)?;
        Ok(file.write_all(content.as_bytes())?)
    }
}

#[derive(Debug)]
pub enum CollectionError {
    IoError,
    InvalidFormat,
    ParsingError,
    CollectionNotFound,
    ItemInCollectionNotFound,
}

impl From<std::io::Error> for CollectionError {
    fn from(_: std::io::Error) -> Self {
        Self::IoError
    }
}

impl From<toml::ser::Error> for CollectionError {
    fn from(_: toml::ser::Error) -> Self {
        CollectionError::ParsingError
    }
}

impl From<CollectionError> for String {
    fn from(value: CollectionError) -> Self {
        match value {
            CollectionError::IoError => "Could not open file".to_string(),
            CollectionError::ParsingError | CollectionError::InvalidFormat => {
                "Invalid file format.".to_string()
            }
            CollectionError::CollectionNotFound => "Collection not found.".to_string(),
            CollectionError::ItemInCollectionNotFound => {
                "Item in collection not found.".to_string()
            }
        }
    }
}
