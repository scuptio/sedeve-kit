use std::collections::VecDeque;
use std::fs::File;
use std::sync::Mutex;

use scupt_util::error_type::ET;
use scupt_util::res::Res;
use scupt_util::res_of::res_io;
use serde_json::Value;

use crate::action::trace::TraceJsonValue;
use crate::player::action_incoming::ActionIncoming;

pub struct ActionInputJson  {
    actions: Mutex<VecDeque<String>>,
}

impl  ActionInputJson {
    pub fn from_json_string(json_string: String) -> Res<Self> {
        let exec = Self::read_from_string(json_string)?;
        let mut actions = vec![];
        for v in exec.actions.iter() {
            actions.push(v.to_string());
        }
        let input = Self {
            actions: Mutex::new(VecDeque::from(actions)),
        };
        Ok(input)
    }

    pub fn from_file(path: String) -> Res<Self> {
        let json_value:Value = Self::read_from_file(path.clone())?;
        Self::from_json_value(json_value)
    }

    fn from_json_value(json_value:Value) -> Res<Self> {
        let t = TraceJsonValue::from_json_value(json_value)?;
        let mut actions = vec![];
        for v in t.actions.iter() {
            actions.push(v.to_string());
        }
        Ok(Self {
            actions: Mutex::new(VecDeque::from(actions)),
        })
    }

    fn read_from_file(path: String) -> Res<Value> {
        let r_open = File::open(&path);
        let file = res_io(r_open)?;

        let r_from_reader = serde_json::from_reader::<_, _>(file);
        let execution = match r_from_reader {
            Ok(e) => { e }
            Err(e) => { return Err(ET::SerdeError(e.to_string())) }
        };
        Ok(execution)
    }

    fn read_from_string(json_string: String) -> Res<TraceJsonValue> {
        let json_value:Value = serde_json::from_str::<_>(json_string.as_str()).unwrap();
        Ok(TraceJsonValue::from_json_value(json_value)?)
    }
}

impl ActionIncoming for ActionInputJson {
    fn next(&self) -> Res<String> {
        let mut action = self.actions.lock().unwrap();
        let opt = action.pop_front();
        match opt {
            Some(e) => { Ok(e) }
            None => { Err(ET::EOF) }
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::sync::Arc;

    use crate::data::path::test::test_data_path;
    use crate::player::action_incoming::ActionIncoming;
    use crate::player::action_input_json::ActionInputJson;

    #[test]
    fn test() {
        let path = test_data_path("trace1.json".to_string()).unwrap();
        let incoming1:Arc<dyn ActionIncoming> = Arc::new(ActionInputJson::from_file(path.clone()).unwrap());
        let string = read_to_string(path).unwrap();
        let incoming2:Arc<dyn ActionIncoming> = Arc::new(ActionInputJson::from_json_string(string).unwrap());

        let path = test_data_path("trace2.json".to_string()).unwrap();
        let incoming3:Arc<dyn ActionIncoming> = Arc::new(ActionInputJson::from_file(path.clone()).unwrap());
        let string = read_to_string(path).unwrap();
        let incoming4:Arc<dyn ActionIncoming> = Arc::new(ActionInputJson::from_json_string(string).unwrap());

        let mut n = 0;
        for incoming in [incoming1, incoming2, incoming3, incoming4] {
            while let Ok(_s) = incoming.next() {
                n += 1;
            }
        }
        assert_eq!(n, 4*4);
    }
}