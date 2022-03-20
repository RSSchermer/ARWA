use std::any::TypeId;

use crate::util::type_id_to_u64;

pub(crate) fn type_id_to_event_name(type_id: TypeId) -> String {
    format!("__ARWA_CUSTOM_EVENT_{}", type_id_to_u64(type_id))
}
