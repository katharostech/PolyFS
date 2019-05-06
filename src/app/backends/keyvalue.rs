use crate::PolyfsResult;

pub trait KeyValueStore {
    fn get(&self, key: &str) -> PolyfsResult<&str>;
    fn set(&self, key: &str) -> PolyfsResult<()>;
    fn list(&self) -> PolyfsResult<&[&str]>;
}
