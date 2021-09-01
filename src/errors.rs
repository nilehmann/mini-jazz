use crate::database::DbError;

pub enum RuntimeError {
    Db(DbError),
    Value,
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;
