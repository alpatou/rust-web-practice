use crate::to_do::enums::Command;

use super::to_do::structs::done::Done;
use super::to_do::structs::pending::Pending;
use super::to_do::traits::create::Create;
use super::to_do::traits::delete::Delete;
use super::to_do::traits::edit::Edit;
use super::to_do::traits::get::Get;
use super::to_do::ItemTypes;
use serde_json::value::Value;
use serde_json::Map;

fn process_pending(item: Pending, command: Command, state: &Map<String, Value>) -> () {
    let mut state: Map<String, Value> = state.clone();

    match command {
        // an enum would be nice here, its is done now
        Command::GET => item.get(&item.super_struct.title, &state),
        Command::CREATE => item.create(
            &item.super_struct.title,
            &item.super_struct.status.stringify(),
            &mut state,
        ),
        Command::EDIT => item.set_to_done(&item.super_struct.title, &mut state),
        _ => println!("command {} not supported", command.stringify()),
    }
}

fn process_done(item: Done, command: Command, state: &Map<String, Value>) -> () {
    let mut state: Map<String, Value> = state.clone();

    match command {
        // an enum would be nice here
        Command::GET => item.get(&item.super_struct.title, &state),
        Command::DELETE => item.delete(
            &item.super_struct.title,
            &item.super_struct.status.stringify(),
            &mut state,
        ),
        Command::EDIT => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("command {} not supported", command.stringify()),
    }
}

pub fn process_input(item: ItemTypes, command: Command, state: &Map<String, Value>) -> () {
    match item {
        ItemTypes::Done(item) => process_done(item, command, state),
        ItemTypes::Pending(item) => process_pending(item, command, state),
    }
}
