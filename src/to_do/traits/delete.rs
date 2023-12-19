use crate::state::write_to_file;
use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

pub trait Delete {
    fn delete(&self, title: &str, status: &str, state: Map<String, Value>) -> () {
        state.remove(title);
        write_to_file("./state.json", &mut state);
        println!("\n\n {} is being deleted", title);
    }
}
