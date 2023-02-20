use crate::domain::Nodeset;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

lazy_static! {
    static ref NODESET_HASHMAP: Mutex<HashMap<String, Nodeset>> = Mutex::new({
        let map = HashMap::new();
        map
    });
}

pub fn get_nodeset_hashmap() -> MutexGuard<'static, HashMap<String, Nodeset>> {
    NODESET_HASHMAP.lock().unwrap()
}
