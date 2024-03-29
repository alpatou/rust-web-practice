use crate::state::read_file;
use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use crate::to_do::{enums::TaskStatus, to_do_factory};
use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use serde_json::value::Value;
use serde_json::Map;
use std::vec::Vec;

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

    pub fn get_state() -> ToDoItems {
        let file_path: &str = "./state.json";

        let state: Map<String, Value> = read_file(file_path);

        let mut array_buffer: Vec<ItemTypes> = Vec::new();

        for (key, value) in state {
            let status = TaskStatus::from_string(value.as_str().unwrap().to_string());

            let item: ItemTypes = to_do_factory(&key, status);

            array_buffer.push(item);
        }

        return ToDoItems::new(array_buffer);
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;
    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body: String = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
