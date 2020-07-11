use crate::payload_storage::payload_storage::{PayloadStorage, TheMap};
use crate::types::{PayloadKeyType, PayloadType, PointOffsetType};
use std::mem;
use std::collections::{HashMap, HashSet};

pub struct SimplePayloadStorage {
    payload: HashMap<PointOffsetType, TheMap<PayloadKeyType, PayloadType>>,
}


impl SimplePayloadStorage {
    pub fn new() -> Self {
        SimplePayloadStorage {
            payload: Default::default(),
        }
    }
}

impl PayloadStorage for SimplePayloadStorage {
    fn assign(&mut self, point_id: PointOffsetType, key: &PayloadKeyType, payload: PayloadType) {
        match self.payload.get_mut(&point_id) {
            Some(point_payload) => {
                point_payload.insert(key.to_owned(), payload);
            },
            None => {
                let mut new_payload = TheMap::default();
                new_payload.insert(key.to_owned(), payload);
                self.payload.insert(point_id, new_payload);
            }
        }
    }

    fn payload(&self, point_id: PointOffsetType) -> TheMap<PayloadKeyType, PayloadType> {
        match self.payload.get(&point_id) {
            Some(payload) => payload.clone(),
            None => TheMap::new()
        }
    }

    fn delete(&mut self, point_id: PointOffsetType, key: &PayloadKeyType) -> Option<PayloadType> {
        let point_payload = self.payload.get_mut(&point_id).unwrap();
        point_payload.remove(key)
    }

    fn drop(&mut self, point_id: PointOffsetType) -> Option<TheMap<PayloadKeyType, PayloadType>> {
        self.payload.remove(&point_id)
    }

    fn wipe(&mut self) {
        self.payload = HashMap::new()
    }
}
