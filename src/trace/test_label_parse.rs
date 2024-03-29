#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use log::error;
    use scupt_util::init_logger;
    use scupt_util::res::Res;
    use serde_json::Value;
    use tracing::trace;
    use crate::action;

    use crate::action::tla_actions::tla_action_var_text_to_json;

    #[test]
    fn test_all_labels() {
        init_logger::logger_setup();
        let test_cases = vec![
                (action::constant::__TEST_TLA_ACTION_VARIABLE_CONST,
                 action::constant::__TEST_NODE_ID_MAP.clone())
        ];
        for (case, map) in test_cases {
            let r = test_parse_label(case.to_string(), map);
            match &r {
                Err(e) => { error!("{} case:{}", e.to_string(), case); }
                _ => {}
            }
            assert!(r.is_ok());
        }
    }

    fn test_parse_label(text: String, dict:HashMap<String, Value>) -> Res<()> {
        let v = tla_action_var_text_to_json(text, dict)?;
        trace!("{:?}", v.to_string());
        Ok(())
    }
}