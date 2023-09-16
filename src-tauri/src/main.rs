// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use norg_engine::agenda::AgendaDay;
use norg_engine::category::Category;
use norg_engine::donna::generate_agenda;
use norg_engine::helper::generate_category_list;
use norg_engine::rosetta::generate_category_struct;
use std::collections::HashMap;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//#[tauri::command]
//fn greet(name: &str) -> String {
//    format!("Hello, {}! You've been greeted from Rust!", name)
//}

#[derive(serde::Serialize)]
struct Response {
    categories: Vec<Category>,
    agenda: Vec<AgendaDay>,
}

#[tauri::command]
fn get_path(path: &str) -> Response {
    let file_list: Vec<String> = generate_category_list(path);
    let mut category_vec: Vec<Category> = Vec::new();
    let mut id_map: HashMap<u32, usize> = HashMap::new();

    for file in file_list {
        category_vec.push(generate_category_struct(file.as_str()));
    }

    for i in 0..category_vec.len() {
        id_map.insert(category_vec[i].id, i);
    }
    // category_vec[id_map.get(&(u32::try_from(6).unwrap())).unwrap().to_owned()].clone();
    // would get category with id=6 which is in index 5 of the vector.

    let agenda: Vec<AgendaDay> = generate_agenda(category_vec.to_owned());

    Response {
        categories: category_vec.to_owned(),
        agenda,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
