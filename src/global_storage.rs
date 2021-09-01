use std::{any::Any, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::database::{Database, DbError};

pub enum StorageError {
    Db(DbError),
    Value,
}

pub struct GlobalStorage<'a> {
    db: &'a dyn Database,
    cache: &'a mut GlobalStorageCache,
    pub(crate) effects: HashMap<String, GlobalEffect>,
}

pub trait PersistentValue: 'static + Serialize + for<'a> Deserialize<'a> {}

pub struct GlobalStorageCache {
    map: HashMap<String, Box<dyn Any>>,
}

pub enum GlobalEffect {
    Modified,
    Deleted,
}

impl<'a> GlobalStorage<'a> {
    pub fn new(db: &'a dyn Database, cache: &'a mut GlobalStorageCache) -> Self {
        Self {
            db,
            cache,
            effects: HashMap::new(),
        }
    }

    pub fn put<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: PersistentValue,
    {
        let key = key.into();
        self.effects.insert(key.clone(), GlobalEffect::Modified);
        self.cache.insert(key, value);
    }

    pub fn borrow_mut<K, V>(&mut self, key: K) -> Option<&V>
    where
        K: Into<String>,
        V: PersistentValue,
    {
        let key = key.into();
        self.populate_cache::<V>(key.clone());
        self.effects.insert(key.clone(), GlobalEffect::Modified);
        self.cache.get(&key)
    }

    pub fn remove<K>(&mut self, key: K)
    where
        K: Into<String>,
    {
        self.effects.insert(key.into(), GlobalEffect::Deleted);
    }

    fn populate_cache<V>(&mut self, key: String)
    where
        V: PersistentValue,
    {
        if self.cache.contains_key(&key) {
            return;
        }
        if let Some(bytes) = self
            .db
            .get_resource(&key)
            .unwrap_or_else(|err| std::panic::panic_any(err))
        {
            let value =
                V::from_bytes(&bytes).unwrap_or_else(|| std::panic::panic_any(StorageError::Value));
            self.cache.insert(key, value);
        }
    }
}

impl GlobalStorageCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn insert<V>(&mut self, key: String, value: V)
    where
        V: PersistentValue,
    {
        self.map.insert(key, Box::new(value));
    }

    pub fn get<V>(&self, key: &str) -> Option<&V>
    where
        V: PersistentValue,
    {
        let any = self.map.get(key)?;
        let value = any
            .downcast_ref::<V>()
            .unwrap_or_else(|| std::panic::panic_any(StorageError::Value));
        Some(value)
    }

    pub fn get_mut<V>(&mut self, key: &str) -> Option<&mut V>
    where
        V: PersistentValue,
    {
        let any = self.map.get_mut(key)?;
        let value = any
            .downcast_mut::<V>()
            .unwrap_or_else(|| std::panic::panic_any(StorageError::Value));
        Some(value)
    }
}

impl PersistentValue for u32 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {}

    fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }
}
