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
}
