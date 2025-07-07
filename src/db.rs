use std::{collections::HashMap, fmt::Display};

pub type Id = u64;

#[derive(Debug)]
pub enum Data {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(value) => write!(f, "\"{}\"", value),
            Self::Int(value) => write!(f, "{}", value),
            Self::Float(value) => write!(f, "{:.3}", value),
            Self::Bool(value) => write!(f, "{}", value),
        }
    }
}

impl Data {
    pub fn from_string(string: String) -> Self {
        if let Ok(value) = string.parse::<f64>() {
            if value.fract() == 0.0 {
                return Data::Int(value as i64);
            }

            return Data::Float(value);
        }

        if let Ok(boolean) = string.parse::<bool>() {
            return Data::Bool(boolean);
        }

        Self::String(string)
    }
}

#[derive(Debug)]
pub struct Db {
    data: HashMap<Id, Data>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    pub fn create(&mut self, id: Id, data: Data) {
        self.data.insert(id, data);
        println!("{:#?}", self.data);
    }

    pub fn read(&self, id: Id) -> Option<&Data> {
        self.data.get(&id)
    }

    pub fn update(&mut self, id: Id, new_data: Data) -> Result<(), String> {
        match self.data.insert(id, new_data) {
            Some(_) => {
                println!("{:#?}", self.data);
                Ok(())
            },
            None => Err(format!("Cannot update, There is no data with the id of: {id}"))
        }
    }

    pub fn delete(&mut self, id: Id) -> Result<(), String> {
        match self.data.remove(&id) {
            Some(_) => Ok(()),
            None => Err(format!("Cannot delete, There is no data with the id of: {id}"))
        }
    }
}
