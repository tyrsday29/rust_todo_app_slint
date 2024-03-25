use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::cell::RefCell;
use crate::serializable_item::UpdateIndex;
use crate::AppWindow;
use crate::TodoItem;
use slint::{ Model, VecModel, SharedString};
use std::rc::Rc;
use bincode::deserialize_from;
use crate::serializable_item::SerializableItem;

pub const PATH:&str = "C:/ProgramData/todo/todo";
const FOLDER_PATH:&str = "C:/ProgramData/todo";
const FILE_PATH:&str = "todo";


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

fn get_file()->BufReader<File>{
    let file= File::open(PATH).unwrap();
      let file=BufReader::new(file);
      file
}

//set the initial to-do list from file
pub fn deserialize_and_set(ui: &AppWindow){
    //create bufreader vector for file
    let file= get_file();
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

pub fn push_and_serialize(passed_string:SharedString, ui: &AppWindow){
    //create TodoItem struct from passed string
    let mut todo_item = TodoItem::new(passed_string);
      //create reference counted pointer to items property in slint ui and push item to vector
      let items_model_rc = ui.get_items();
      let items_model: &VecModel<TodoItem> = items_model_rc.as_any().downcast_ref::<VecModel<TodoItem>>().expect("push error!");
          todo_item.indexp = items_model.row_count() as i32;
          items_model.push(todo_item);
      //write the vector to file as binary
      TodoItem::serialize_model_to_file(items_model).expect("push write error!");
 }


pub fn remove_and_serialize(ui: &AppWindow){
    //create reference counted pointer to items property in slint ui
    let items_model_rc = ui.get_items();
    let items_model: &VecModel<TodoItem> = items_model_rc.as_any().downcast_ref::<VecModel<TodoItem>>().expect("remove error!");
    // Iterate over the items and find the ones that are checked from the vector
    TodoItem::remove_checked_items(items_model,ui);
    //write the vector to file as binary  
    TodoItem::serialize_model_to_file(items_model).expect("remove write error!");
 }


 pub fn do_the_move(moovple: Rc<RefCell<Vec<i32>>>, ui: &AppWindow){
    //only acts when there are two items in the moovple
    if moovple.borrow().len()==2{
      let file = get_file();
          
           //If deserialization from file is successful, do stuff, otherwise, don't
           match deserialize_from::<_, Vec<SerializableItem>>(file){
             Ok(mut items) => {
                                  //get the item to be moved from the vector, remove it, reinsert and clear the moovple
                                  let moved_item: SerializableItem = items.get(moovple.borrow()[0] as usize).unwrap().clone();
                                  items.remove(moovple.borrow()[0] as usize);
                                  items.insert(moovple.borrow()[1] as usize, moved_item);
                                  moovple.borrow_mut().clear();
                                  //sets the indexp property to each items index after the move
                                  items.update_index();
                                  //write the corrected vector to the file and the front end
                                  SerializableItem::serialize_to_file(&items).expect("writing failed!");
                                  SerializableItem::write_to_frontend(items, ui);
                              },

             //don't panic!
             Err(_) => (),
            }
      }
 }

 
 pub fn save_edit(passed_item:SharedString, passed_index: i32){
  //deserialize the vector from the file
  let file = get_file();
           //If deserialization is successful, edit the correct vector and write it to file, otherwise, don't (like with a blank file)
           match deserialize_from::<_, Vec<SerializableItem>>(file){
            Ok(mut items)=>{
                items[passed_index as usize].name = passed_item.to_string();
                SerializableItem::serialize_to_file(&items).expect("writing failed");
            },
            //don't panic!
            Err(_) => (),
           }
 }

 pub fn check_all(ui: &AppWindow){
    let file= get_file();
    match deserialize_from::<_, Vec<SerializableItem>>(file){
        //read from file and if it is succesfull change item checked property then write to front and backend
        Ok(mut items)=>{
            items.iter_mut().for_each(|item| item.checked=!item.checked);
            SerializableItem::serialize_to_file(&items).expect("writing failed");
            SerializableItem::write_to_frontend(items, ui);
        },
        //don't panic!
        Err(_) => (),
       }
}

