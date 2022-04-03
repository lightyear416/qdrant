use crate::types::{Payload, PayloadKeyTypeRef, PointOffsetType};
use std::collections::HashMap;

use serde_json::Value;

use crate::entry::entry_point::OperationResult;
use crate::payload_storage::PayloadStorage;


/// Same as `SimplePayloadStorage` but without persistence
/// Warn: for tests only
pub struct InMemoryPayloadStorage {
    payload: HashMap<PointOffsetType, Payload>,
}

impl InMemoryPayloadStorage {
    pub fn new() -> Self {
        InMemoryPayloadStorage {
            payload: Default::default()
        }
    }
}

impl PayloadStorage for InMemoryPayloadStorage {
    fn assign(&mut self, point_id: PointOffsetType, payload: &Payload) -> OperationResult<()> {
        match self.payload.get_mut(&point_id) {
            Some(point_payload) => point_payload.merge(payload),
            None => {
                self.payload.insert(point_id, payload.to_owned());
            }
        }
        Ok(())
    }

    fn payload(&self, point_id: PointOffsetType) -> Payload {
        match self.payload.get(&point_id) {
            Some(payload) => payload.to_owned(),
            None => Default::default(),
        }
    }

    fn delete(
        &mut self,
        point_id: PointOffsetType,
        key: PayloadKeyTypeRef,
    ) -> OperationResult<Option<Value>> {
        match self.payload.get_mut(&point_id) {
            Some(payload) => {
                let res = payload.remove(key);
                Ok(res)
            }
            None => Ok(None),
        }
    }

    fn drop(&mut self, point_id: PointOffsetType) -> OperationResult<Option<Payload>> {
        let res = self.payload.remove(&point_id);
        Ok(res)
    }

    fn wipe(&mut self) -> OperationResult<()> {
        self.payload = HashMap::new();
        Ok(())
    }

    fn flush(&self) -> OperationResult<()> {
        Ok(())
    }

    fn iter_ids(&self) -> Box<dyn Iterator<Item = PointOffsetType> + '_> {
        Box::new(self.payload.keys().copied())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wipe() {
        let mut storage = InMemoryPayloadStorage::new();
        let payload: Payload = serde_json::from_str(r#"{"name": "John Doe"}"#).unwrap();
        storage.assign(100, &payload).unwrap();
        storage.wipe().unwrap();
        storage.assign(100, &payload).unwrap();
        storage.wipe().unwrap();
        storage.assign(100, &payload).unwrap();
        assert!(!storage.payload(100).is_empty());
        storage.wipe().unwrap();
        assert_eq!(storage.payload(100), Default::default());
    }

    #[test]
    fn test_assign_payload_from_serde_json() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "boolean": "true",
            "floating": 30.5,
            "string_array": ["hello", "world"],
            "boolean_array": ["true", "false"],
            "float_array": [1.0, 2.0],
            "integer_array": [1, 2],
            "geo_data": {"type": "geo", "value": {"lon": 1.0, "lat": 1.0}},
            "metadata": {
                "height": 50,
                "width": 60,
                "temperature": 60.5,
                "nested": {
                    "feature": 30.5
                },
                "integer_array": [1, 2]
            }
        }"#;

        let payload: Payload = serde_json::from_str(data).unwrap();
        let mut storage = InMemoryPayloadStorage::new();
        storage.assign(100, &payload).unwrap();
        let pload = storage.payload(100);
        assert_eq!(pload, payload);
    }
}
