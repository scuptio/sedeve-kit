use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use scupt_util::error_type::ET;
use scupt_util::message::{Message, MsgTrait};
use scupt_util::node_id::NID;
use scupt_util::res::Res;
use scupt_util::res_of::res_option;
use scupt_util::serde_json_value::SerdeJsonValue;
use serde_json::{json, Value};

use crate::action::action_message::ActionMessage;
use crate::action::action_type::ActionType;
use crate::action::message_json::MessageJson;
use crate::action::res_serde::res_serde;

#[derive(Clone, Debug)]
pub struct ActionJson {
    value: SerdeJsonValue,
}


impl Deref for ActionJson {
    type Target = SerdeJsonValue;

    fn deref(&self) -> &SerdeJsonValue {
        &self.value
    }
}

impl DerefMut for ActionJson {
    fn deref_mut(&mut self) -> &mut SerdeJsonValue {
        &mut self.value
    }
}

impl ActionJson {
    pub fn from_value(value: Value) -> Self {
        Self {
            value: SerdeJsonValue::new(value),
        }
    }

    pub fn from_json(
        action_type: ActionType,
        source: NID,
        dest: NID,
        json_payload: String,
    ) -> Res<Self> {
        let v: Value = Message::<()>::build_json_value(json_payload, source, dest)?;
        let json = json!({
            action_type.to_string():
            v
        });
        Self::from_json_value(json)
    }
    pub fn from_message<M: MsgTrait + 'static>(
        action_type: ActionType,
        message: Message<M>,
    ) -> Res<Self> {
        let action_message = ActionMessage::from_message(action_type, message);
        let r = serde_json::to_value(action_message);
        let value = res_serde(r)?;
        Self::from_json_value(value)
    }

    pub fn from_json_value(json: Value) -> Res<Self> {
        Ok(Self {
            value: SerdeJsonValue::new(json),
        })
    }

    pub fn message_payload_json_value(&self) -> Res<&Value> {
        let opt = self.value.serde_json_value_ref().as_object();
        let map = res_option(opt)?;
        for (_k, v) in map.iter() {
            return Ok(v);
        }
        return Err(ET::SerdeError("error action format".to_string()));
    }

    pub fn action_type(&self) -> Res<ActionType> {
        let map = res_option(self.value.serde_json_value_ref().as_object())?;
        for k in map.keys() {
            return Ok(ActionType::from_serde_action_type(k.as_str()));
        }
        Err(ET::JSONError("no action type".to_string()))
    }

    pub fn source_nid(&self) -> Res<NID> {
        self.handle_object_one_key_value(|_k, v| {
            let ms = MessageJson::new(v);
            ms.source_nid()
        })
    }

    pub fn dest_nid(&self) -> Res<NID> {
        self.handle_object_one_key_value(|_k, v| {
            let ms = MessageJson::new(v);
            ms.dest_nid()
        })
    }

    fn handle_object_one_key_value<F, R>(&self, f: F) -> Res<R>
        where F: Fn(&String, &Value) -> Res<R>
    {
        let map = res_option(self.value.serde_json_value_ref().as_object())?;
        if map.len() != 1 {
            return Err(ET::SerdeError("json format error".to_string()));
        }
        for (k, v) in map.iter() {
            return f(k, v);
        }
        return Err(ET::SerdeError("json format error".to_string()));
    }
}

impl Hash for ActionJson {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for ActionJson {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.value.eq(&other.value)
    }
}

impl Eq for ActionJson {}


#[cfg(test)]
mod test {
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};

    use bincode::{Decode, Encode};
    use scupt_util::message::{Message, MsgTrait};
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use crate::action::action_json::ActionJson;
    use crate::action::action_message::ActionMessage;

    #[derive(
    Clone,
    Serialize,
    Hash,
    PartialEq,
    Eq,
    Debug,
    Deserialize,
    Decode,
    Encode,
    )]
    struct M {
        v: i32,
    }

    impl MsgTrait for M {}

    #[test]
    fn test_action_json() {
        let mut map = HashMap::new();
        for i in 0..10i32 {
            ActionMessage::Input(Message::<M>::new(M { v: 1 }, 2, 1));
            let j = ActionJson::from_json_value(Value::from(i)).unwrap();
            let n = map.insert(j, i);
            assert!(n.is_none());
        }
    }

    #[test]
    fn test_action_map_order() {
        let s1 = r#"
{
   "Input":{
      "source":1,
      "dest":2,
      "payload":{
         "DTMTesting":{
            "HandleAppendLog":{
               "term":2,
               "log_entries":[
                  {
                     "term":2,
                     "index":1,
                     "value":100
                  }
               ],
               "prev_log_index":0,
               "prev_log_term":0,
               "commit_index":0
            }
         }
      }
   }
}"#;
        let s2 = r#"
{
   "Input":{
      "source":1,
      "dest":2,
      "payload":{
         "DTMTesting":{
            "HandleAppendLog":{
               "term":2,
               "prev_log_index":0,
               "prev_log_term":0,
               "log_entries":[
                  {
                     "term":2,
                     "index":1,
                     "value":100
                  }
               ],
               "commit_index":0
            }
         }
      }
   }
}"#;
        let v1 = serde_json::from_str(s1).unwrap();
        let v2 = serde_json::from_str(s2).unwrap();
        let a1 = ActionJson::from_json_value(v1).unwrap();
        let a2 = ActionJson::from_json_value(v2).unwrap();
        assert_eq!(a1, a2);
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        a1.hash(&mut h1);
        a2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }
}