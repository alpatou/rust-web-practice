mod processes;
use processes::process_input;
mod state;
use serde_json::value::Value;
use serde_json::{json, Map};
use state::{read_file, write_to_file};
use std::env;
mod to_do;
use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;
use to_do::enums::TaskStatus;
use to_do::to_do_factory;
use to_do::ItemTypes;

fn main() {
    let file_path: String = String::from("./state.json");
    let mut state: Map<String, Value> = read_file(&file_path);

    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];
    let status: String;

    status = match &state.get(title) {
        Some(result) => result.to_string().replace('\"', ""),
        None => "pending".to_owned(),
    };

    let task: ItemTypes = to_do_factory(title, TaskStatus::from_string(status.to_uppercase()));
    process_input(task, command.to_string(), &state);

    println!("Before ops state was {:?}", state);
    state.insert(title.to_string(), json!(status));

    println!("After operation: {:?}", state);

    write_to_file(&file_path, &mut state);
}
