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
    let done: ItemTypes = to_do::to_do_factory("shopping", TaskStatus::PENDING);
    match (done) {
        ItemTypes::Done(done_task) => {
            done_task.get(&done_task.super_struct.title);
            done_task.delete(&done_task.super_struct.title);
        }
        ItemTypes::Pending(pending_task) => {
            pending_task.get(&pending_task.super_struct.title);
            pending_task.set_to_done(&pending_task.super_struct.title);
        }
    }

    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];
    let file_path: String = String::from("./state.json");
    let mut state: Map<String, Value> = read_file(&file_path);

    println!("Before ops state was {:?}", state);
    state.insert(title.to_string(), json!(status));

    println!("After operation: {:?}", state);

    write_to_file(&file_path, &mut state);
}
