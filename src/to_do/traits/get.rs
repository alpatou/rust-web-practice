use serde_json::value::Value;
use serde_json::Map;
pub trait Get {
    fn get(&self, title: &str, state: Map<String, Value>) -> () {
        let item: Option<&Value> = state.get(title);

        match item {
            Some(value) => {
                println!("\n\n Item: {}", title);
                println!("Status: {} \n \n ", value)
            }
            None => println!("{} Item was not found ", title),
        }
    }
}
