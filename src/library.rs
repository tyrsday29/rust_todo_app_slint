use std::fs::File;
use std::path::Path;
use std::io::{BufWriter, BufReader};
//this includes the TodoItem struct exported from slint
slint::include_modules!();
use serde::{Deserialize, Serialize};
use slint::{ Model, VecModel, SharedString};
use std::rc::Rc;
use bincode::deserialize_from;


pub const PATH:&str = "C:/ProgramData/todo/todo";
const FOLDER_PATH:&str = "C:/ProgramData/todo";
const FILE_PATH:&str = "todo";


#[derive(Serialize, Deserialize)]
 struct SerializableItem{
   name: String,
   checked: bool,
}

impl From<TodoItem> for SerializableItem {
    fn from(item:TodoItem) -> Self{
        SerializableItem {
          name: item.name.to_string(),
          checked: item.checked,
        }
    }
}

impl From<SerializableItem> for TodoItem{
    fn from(item:SerializableItem) -> Self{
        TodoItem{
            name: item.name.into(),
            checked: item.checked,
        }
    }
}

impl TodoItem {
    
    fn write_and_serialize(the_model: &VecModel<Self>) -> Result<(), Box<dyn std::error::Error>> {
        // Creates bufwriter vector for file to write to
        let file = File::create(PATH).unwrap();
        let file = BufWriter::new(file);
        // Create vector of SerializableItems from the TodoItems to simply deal with Shared Strings
        let items: Vec<SerializableItem> = the_model.iter().map(|item| item.into()).collect();
        // Write to the file in binary                                          
        bincode::serialize_into(file, &items).expect("Failed to write to file");

        Ok(())
    }


    pub fn remove_checked_items(items_model: &VecModel<Self>) {
        // Iterate over the items and find the ones that are checked from the vector
        let checked_items: Vec<_> = items_model.iter().enumerate().filter(|(_, item)| item.checked).collect();
        for (index, _) in checked_items.into_iter().rev() {
            items_model.remove(index);
        }
    }
}


//creates the necessary folders and files if they do not exist, or if something happens to them only for windows machines
pub fn first_start(){
  let folder_path = Path::new(FOLDER_PATH);
  let file_path = folder_path.join(FILE_PATH);
    //checks for the folder and file path create them if necessary
    if !folder_path.exists() {
            std::fs::create_dir_all(&folder_path).expect("there's no folder and I can't make it");
        }
    if !file_path.exists() {
            File::create(&file_path).expect("there's no file and I can't make it");
        }
}


//set the initial to-do list from file
pub fn deserialize_and_set(ui: &AppWindow){
    //create bufreader vector for file
    let file= File::open(PATH).unwrap();
    let file=BufReader::new(file);
         //If deserialization is successful, do stuff, otherwise, don't (like with a blank file)
         match deserialize_from::<_, Vec<SerializableItem>>(file){
           Ok(items) => {
                                //change into correct vector struct, and to SharedString
                                let items: Vec<TodoItem> = items.into_iter().map(|item|item.into()).collect(); 
                                //make reference counted pointer and save to ui
                                let items:Rc<VecModel<TodoItem>>=Rc::new(VecModel::from(items));
                                ui.set_items(items.into());        
                            },
           //don't panic!
           Err(_) => (),
     };
 }


//create new struct of TodoItem passed from the .slint file
fn create_item(passed_string:SharedString)->TodoItem{
        TodoItem{
            name: passed_string,
            checked: false,}        }


pub fn push_and_serialize(passed_string:SharedString, ui: &AppWindow){
    //create TodoItem struct from passed string
    let todo_item = create_item(passed_string);
      //create reference counted pointer to items property in slint ui and push item to vector
      let items_model_rc = ui.get_items();
      let items_model: &VecModel<TodoItem> = items_model_rc.as_any().downcast_ref::<VecModel<TodoItem>>().expect("push error!");
          items_model.push(todo_item);
      //write the vector to file as binary
      TodoItem::write_and_serialize(items_model).expect("push write error!");
 }


pub fn remove_and_serialize(ui: &AppWindow){
    //create reference counted pointer to items property in slint ui
    let items_model_rc = ui.get_items();
    let items_model: &VecModel<TodoItem> = items_model_rc.as_any().downcast_ref::<VecModel<TodoItem>>().expect("remove error!");
    // Iterate over the items and find the ones that are checked from the vector
    TodoItem::remove_checked_items(items_model);
    //write the vector to file as binary  
    TodoItem::write_and_serialize(items_model).expect("remove write error!");
 }