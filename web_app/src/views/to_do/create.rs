use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::ItemTypes;
use crate::to_do::{enums::TaskStatus, to_do_factory};
use actix_web::HttpRequest;
use serde_json::value::Value;
use serde_json::Map;

pub async fn create ( req: HttpRequest) -> String {

   let file_path : &str = "./state.json";

   let state : Map<String, Value> = read_file(file_path);

   let title : String = req.match_info().get("title").unwrap().to_string();

   let item : ItemTypes = to_do_factory(&title, TaskStatus::PENDING );

   process_input(item,"create".to_string() ,&state );

   format!("{} created", title)

}
