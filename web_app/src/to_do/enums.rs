use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;

pub enum Command {
    GET,
    CREATE,
    EDIT,
    DELETE,
}

impl Command {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::GET => "GET".to_string(),
            &Self::CREATE => "CREATE".to_string(),
            &Self::EDIT => "EDIT".to_string(),
            &Self::DELETE => "DELETE".to_string(),
        }
    }

    pub fn from_string(command: String) -> Self {
        match command.as_str() {
            "CREATE" => Self::CREATE,
            "EDIT" => Self::EDIT,
            "DELETE" => Self::DELETE,
            _ => panic!("command {} not supported", command),
        }
    }
}

pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => "DONE".to_string(),

            &Self::PENDING => "PENDING".to_string(),
        }
    }

    pub fn from_string(status: String) -> Self {
        match status.as_str() {
            "DONE" => Self::DONE,
            "PENDING" => Self::PENDING,
            _ => panic!("{} not supported", status),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self) {
            &Self::DONE => write! {f,"DONE"},
            &Self::PENDING => write! {f,"PENDING"},
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(serializer.serialize_str(&self.stringify().as_str()))?
    }
}
