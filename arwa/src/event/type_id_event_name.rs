use std::any::TypeId;
use std::hash::{Hash, Hasher};

struct TypeIdHasher {
    result: u64,
}

impl TypeIdHasher {
    fn new() -> Self {
        TypeIdHasher { result: 0 }
    }
}

impl Hasher for TypeIdHasher {
    fn finish(&self) -> u64 {
        self.result
    }

    fn write(&mut self, bytes: &[u8]) {
        for (i, byte) in bytes.iter().take(8).enumerate() {
            self.result += (*byte as u64) << (i as u64);
        }
    }

    fn write_u64(&mut self, value: u64) {
        self.result = value;
    }
}

pub(crate) fn type_id_to_event_name(type_id: TypeId) -> String {
    let mut type_id_hasher = TypeIdHasher::new();

    type_id.hash(&mut type_id_hasher);

    let type_num = type_id_hasher.finish();

    format!("__ARWA_CUSTOM_EVENT_{}", type_num)
}
