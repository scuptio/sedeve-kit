use scupt_util::node_id::NID;
use scupt_util::res::Res;
use scupt_util::res_of::res_option;
use serde_json::Value;

use crate::action::constant::{
    MESSAGE_FIELD_DEST,
    MESSAGE_FIELD_PAYLOAD,
    MESSAGE_FIELD_SOURCE,
};
use crate::action::serde_json_util::json_util_map_get_value;

#[derive(Debug)]
pub struct MessageJson<'v> {
    value: &'v Value,
}


impl<'v> MessageJson<'v> {
    pub fn new(v: &'v Value) -> Self {
        Self { value: v }
    }

    pub fn source_nid(&self) -> Res<NID> {
        Self::get_node_id(self.value, MESSAGE_FIELD_SOURCE)
    }


    pub fn dest_nid(&self) -> Res<NID> {
        Self::get_node_id(self.value, MESSAGE_FIELD_DEST)
    }

    pub fn payload(&self) -> Res<Value> {
        let map = res_option(self.value.as_object())?;
        let v = json_util_map_get_value(map, MESSAGE_FIELD_PAYLOAD)?;
        Ok(v)
    }

    fn get_node_id(value: &Value, key: &str) -> Res<NID> {
        let map = res_option(value.as_object())?;
        let v = json_util_map_get_value(map, key)?;
        let nid = res_option(v.as_u64())? as NID;
        Ok(nid)
    }
}