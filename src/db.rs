use std::{collections::HashMap, fmt::Display};

pub type Id = u64;

#[derive(Debug)]
pub enum DataValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl Display for DataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(value) => write!(f, "\"{}\"", value),
            Self::Int(value) => write!(f, "{}", value),
            Self::Float(value) => write!(f, "{:.3}", value),
            Self::Bool(value) => write!(f, "{}", value),
        }
    }
}

impl DataValue {
    pub fn from_string(string: String) -> Self {
        if let Ok(value) = string.parse::<f64>() {
            if value.fract() == 0.0 {
                return Self::Int(value as i64);
            }

            return Self::Float(value);
        }

        if let Ok(boolean) = string.parse::<bool>() {
            return Self::Bool(boolean);
        }

        Self::String(string)
    }
}

#[derive(Debug)]
pub struct Data {
    pub value: DataValue,
    pub chance: f32,
}

impl Data {
    pub fn new(value: DataValue) -> Self {
        Self {
            value,
            chance: rand::random(),
        }
    }
}

#[derive(Debug)]
pub struct Db {
    data: HashMap<Id, Data>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn create(&mut self, data: Data) {
        // Random chance that it forgots to even add.
        if rand::random::<f32>() > 0.9 {
            return;
        }

        let id: Id = rand::random();
        self.data.insert(id, data);

        println!("INSERT = {:#?}", self.data);
    }

    pub fn read(&self, id: Id) -> Option<&Data> {
        // Random chance that it just forgots a bit but no completely?
        if rand::random::<f32>() > 0.9 {
            return None;
        }

        let data = self.data.get(&id);
        println!("READ = {:#?}", self.data);

        data
    }

    pub fn list(&self) -> Vec<(&Id, &Data)> {
        let mut list = Vec::new();

        for (id, data) in self.data.iter() {
            list.push((id, data));
        }

        list
    }

    pub fn update(&mut self, id: Id, new_data: Data) -> Result<(), String> {
        match self.data.insert(id, new_data) {
            Some(_) => {
                println!("UPDATE = {:#?}", self.data);
                Ok(())
            },
            None => Err(format!("Cannot update, There is no data with the id of: {id}"))
        }
    }

    pub fn delete(&mut self, id: Id) -> Result<(), String> {
        match self.data.remove(&id) {
            Some(_) => {
                println!("DELETE = {:#?}", self.data);
                Ok(())
            },
            None => Err(format!("Cannot delete, There is no data with the id of: {id}"))
        }
    }

    pub fn forget_random(&mut self) {
        if self.data.len() <= 0 {
            return;
        }

        let data_keys = self.data.keys().cloned().collect::<Vec<Id>>();

        let random_index = rand::random_range(0..self.data.len());
        let random_key = data_keys[random_index];
        let random_data = self.data.get(&random_key).unwrap();

        if rand::random::<f32>() > random_data.chance {
            println!("deleted: {}", random_key);
            self.data.remove(&random_key);
        }
    }
}
