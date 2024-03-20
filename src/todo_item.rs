use crate::functionslib::PATH;
use crate::serializable_item::{SerializableItem, Item, UpdateIndex};
use slint::VecModel;
use std::fs::File;
use std::io::BufWriter;
use std::rc::Rc;
use slint::{Model, SharedString};
use crate::TodoItem;
use crate::AppWindow;

impl From<TodoItem> for SerializableItem {
    fn from(item:TodoItem) -> Self{
        SerializableItem {
          name: item.name.to_string(),
          checked: item.checked,
          indexp: item.indexp,
        }
    }
}
impl Item for TodoItem{
    fn set_index(&mut self, index:i32){
        self.indexp = index;
    }
}

impl TodoItem {

   pub fn serialize_model_to_file(the_model: &VecModel<Self>) -> Result<(), Box<dyn std::error::Error>> {
        // Creates bufwriter vector for file to write to
        let file = File::create(PATH).unwrap();
        let file = BufWriter::new(file);
        // Create vector of SerializableItems from the TodoItems to simply deal with Shared Strings
        let items: Vec<SerializableItem> = the_model.iter().map(|item| item.into()).collect();
        // Write to the file in binary                                          
        bincode::serialize_into(file, &items).expect("Failed to write to file");
        Ok(())
    }

    pub fn remove_checked_items(items_model: &VecModel<Self>, ui: &AppWindow) {
        // Iterate over the items and find the ones that are checked from the vector
        let checked_items: Vec<_> = items_model.iter().enumerate().filter(|(_, item)| item.checked).collect();
        for (index, _) in checked_items.into_iter().rev() {
            items_model.remove(index);
        }
        //turn to a vec of TodoItems and update the indexp property for proper item switching
        let mut vec: Vec<TodoItem> = items_model.iter().collect();
        vec.update_index();
        //turn into a vecmodel and set items list68
        let vec:Rc<VecModel<TodoItem>>=Rc::new(VecModel::from(vec));
        ui.set_items(vec.into());
    }

   pub fn new(passed_string:SharedString)->Self{
        TodoItem{
            name: passed_string,
            checked: false,
            indexp: 0,
        }
    }
}
