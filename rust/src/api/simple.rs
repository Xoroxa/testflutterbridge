// use snips_nlu_lib::ontology::{IntentClassifierResult, IntentParserAlternative, Slot};
use snips_nlu_lib::ontology::IntentParserResult;
use snips_nlu_lib::SnipsNluEngine;
// use snips_nlu_ontology::IntentParserResult;

pub struct NLU {
    inner: SnipsNluEngine,
}

// pub struct IntentParserResult {
//     pub input: String,
//     pub intent: IntentClassifierResult,
//     pub slots: Vec<Slot>,
//     #[serde(default)]
//     pub alternatives: Vec<IntentParserAlternative>,
// }

impl NLU {
    pub fn new(engine_dir: &str) -> Self {
        let engine = SnipsNluEngine::from_path(engine_dir).unwrap();
        NLU {
            inner: engine,
        }
    }

    pub async fn parse_intent(&self, input: String) -> snips_nlu_lib::ontology::IntentParserResult {

        self.inner.parse_with_alternatives(input.trim(), None, None, 0, 0).unwrap()
        // let result = self.inner.parse_with_alternatives(input.trim(), None, None, 0, 0).unwrap();
        // let result_json = serde_json::to_string_pretty(&result).unwrap();
        // serde_json::from_str(&result_json).unwrap()
    }
}


#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
