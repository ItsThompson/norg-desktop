// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use norg_engine::agenda::{AgendaDay, AgendaType};
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
struct AgendaDayResponse {
    date: String,
    items: Vec<String>,
}

#[tauri::command]
fn generate_text(path: &str) -> Vec<AgendaDayResponse> {
    let file_list: Vec<String> = generate_category_list(path);
    let mut category_vec: Vec<Category> = Vec::new();
    let mut id_map: HashMap<u32, usize> = HashMap::new();
    // category_vec[id_map.get(&(u32::try_from(6).unwrap())).unwrap().to_owned()].clone();
    // would get category with id=6 which is in index 5 of the vector.

    for file in file_list {
        category_vec.push(generate_category_struct(file.as_str()));
    }

    for i in 0..category_vec.len() {
        id_map.insert(category_vec[i].id, i);
    }
    let agenda: Vec<AgendaDay> = generate_agenda(category_vec.to_owned());

    let out: Vec<AgendaDayResponse> = generate_agenda_day_response(agenda);
    out
}

fn generate_agenda_day_response(agenda: Vec<AgendaDay>) -> Vec<AgendaDayResponse> {
    let mut agenda_day_response_vec: Vec<AgendaDayResponse> = Vec::new();

    for agenda_day in agenda.to_owned().iter() {
        let date_string = agenda_day.date.get_date_string();
        let mut items_vec: Vec<String> = Vec::new();
        match agenda_day.to_owned().agenda_items {
            Some(_) => {
                let agenda_items = agenda_day.to_owned().agenda_items.unwrap();
                for agenda_item in agenda_items.iter() {
                    let agenda_type = agenda_item.to_owned().agenda_type;
                    match agenda_type {
                        AgendaType::ScheduleItem(schedule_item) => {
                            let start_time: String = schedule_item.date.start_time_string();
                            let end_time: String = schedule_item.date.end_time_string();
                            items_vec.push(format!(
                                "{}-{} (Scheduled) {}",
                                start_time, end_time, schedule_item.name
                            ));
                        }
                        AgendaType::TodoItem(todo_item) => {
                            let start_time: String = todo_item.date.start_time_string();
                            let end_time: String = todo_item.date.end_time_string();
                            items_vec.push(format!(
                                "{}-{} (Todo) {}",
                                start_time, end_time, todo_item.title
                            ));
                        }
                    }
                }
            }
            None => {
                items_vec.push(String::from("None"));
            }
        }

        agenda_day_response_vec.push(AgendaDayResponse {
            date: date_string,
            items: items_vec,
        })
    }
    agenda_day_response_vec
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
