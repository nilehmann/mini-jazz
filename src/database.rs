use serde::{Deserialize, Serialize};

pub type Bytes = Vec<u8>;

pub struct DbError;

pub type DbResult<T> = Result<T, DbError>;

pub struct Database;

impl Database {
    pub fn get_resource<V>(&self, key: &str) -> DbResult<Option<V>>
    where
        V: PersistentValue,
    {
        todo!()
    }

    pub fn update_resource<V>(&mut self, key: String, value: V) -> DbResult<bool>
    where
        V: PersistentValue,
    {
        todo!()
    }
}

pub trait PersistentValue: 'static + Serialize + for<'a> Deserialize<'a> {}

impl PersistentValue for u32 {}

impl PersistentValue for bool {}
