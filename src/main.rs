#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
use slint::{SharedString, ComponentHandle};
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;
use lib::AppWindow;
use lib::functionslib::{check_all, deserialize_and_set, do_the_move, first_start, push_and_serialize, remove_and_serialize, save_edit, PATH};

fn main() -> Result<(), slint::PlatformError> {
  let ui: AppWindow = AppWindow::new()?;
  let ui_handle = ui.as_weak();
   let ui_clone_handle= ui_handle.clone();                                              
   let ui_clone_handle0= ui_handle.clone();
   let ui_clone_handle1= ui_handle.clone();
  let moovple = Rc::new(RefCell::new(Vec::new()));

//create writer file if one doesn't exist
 if !Path::new(PATH).exists(){ first_start();}
//set the contents of the window
  deserialize_and_set(&ui);
//enter a todo
ui.on_enter_todo( move |passed_item:SharedString| {
    let ui = ui_handle.unwrap();
      push_and_serialize(passed_item, &ui) }  );
//delete a checked item
ui.on_completed( move ||{
    let ui = ui_clone_handle.unwrap();
     remove_and_serialize(&ui) }   );
//switching items
ui.on_move( move |passed_index:i32|{
    let ui = ui_clone_handle0.unwrap();
    moovple.borrow_mut().push(passed_index);
    do_the_move(moovple.clone(), &ui); }  );
//editing items
ui.on_pass_back( move |passed_item, passed_index| {
      save_edit(passed_item, passed_index);
    });
//check all box
ui.on_supercheck(move||{
  let ui = ui_clone_handle1.unwrap();
  check_all(&ui);
});

    ui.run()
}