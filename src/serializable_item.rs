use serde::{Serialize, Deserialize};
use crate::functionslib::PATH;
use std::fs::File;
use std::io::BufWriter;
use std::rc::Rc;
use slint::VecModel;
use crate::{TodoItem, AppWindow};

#[derive(Serialize, Deserialize, Clone, Debug)]
 pub struct SerializableItem{
   pub name: String,
   pub checked: bool,
   pub indexp: i32,
}

pub trait Item{
   fn set_index(&mut self, index:i32);
}

pub trait UpdateIndex{
    fn update_index(&mut self);
}

impl Item for SerializableItem{
    fn set_index(&mut self, index:i32) {
        self.indexp = index;
    }
}

impl <T: Item> UpdateIndex for Vec<T>{
    fn update_index(&mut self) {
        for (i, item) in self.iter_mut().enumerate() {
            item.set_index(i as i32)
        }
    }
}

impl From<SerializableItem> for TodoItem{
    fn from(item:SerializableItem) -> Self{
        TodoItem{
            name: item.name.into(),
            checked: item.checked,
            indexp: item.indexp,
        }
    }
}

impl SerializableItem{
   pub fn write_to_frontend(vector: Vec<Self>, ui: &AppWindow){
    let items: Vec<TodoItem> = vector.into_iter().map(|item|item.into()).collect(); 
    //make reference counted pointer and save to ui
    let items:Rc<VecModel<TodoItem>>=Rc::new(VecModel::from(items));
    ui.set_items(items.into());        
   }


   pub fn serialize_to_file(the_model: &Vec<SerializableItem>)-> Result<(), Box<dyn std::error::Error>>{
         // Creates bufwriter vector for file to write to
         let file = File::create(PATH).unwrap();
         let file = BufWriter::new(file);
         bincode::serialize_into(file, the_model).expect("Failed to write to file");
         Ok(())
    }
}