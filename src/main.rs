#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
mod library;
use library::{ push_and_serialize, remove_and_serialize, deserialize_and_set, AppWindow, PATH, first_start};
use slint::{SharedString, ComponentHandle};
use std::path::Path;

fn main() -> Result<(), slint::PlatformError> {
  let ui: AppWindow = AppWindow::new()?;
   let ui_handle = ui.as_weak();
   let ui_clone_handle= ui_handle.clone();                                              
    
  if !Path::new(PATH).exists(){
        first_start();  }

  deserialize_and_set(&ui);

ui.on_enter_todo(move |passed_item:SharedString| {
    let ui = ui_handle.unwrap();
      push_and_serialize(passed_item, &ui) }  );

ui.on_completed(move || {
    let ui = ui_clone_handle.unwrap();
     remove_and_serialize(&ui) }   );

//run the AppWindow
    ui.run()
}