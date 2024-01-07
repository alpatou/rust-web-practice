use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use serde::Serialize;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_array_buffer: Vec<Base> = Vec::new();
        let mut done_array_buffer: Vec<Base> = Vec::new();
        for item in input_items {
            match item {
                ItemTypes::Done(done_item) => done_array_buffer.push(done_item.super_struct),
                ItemTypes::Pending(pending_item) => {
                    pending_array_buffer.push(pending_item.super_struct)
                }
            }
        }
        let done_count: i8 = done_array_buffer.len() as i8;
        let pending_count: i8 = pending_array_buffer.len() as i8;

        return ToDoItems {
            pending_items: pending_array_buffer,
            done_items: done_array_buffer,
            pending_item_count: pending_count,
            done_item_count: done_count,
        };
    }
}
