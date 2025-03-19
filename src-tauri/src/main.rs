// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
fn main() {
    my_project_lib::run()
    // utils::sqlite::createdb().unwrap();
    // utils::post::insert_data(
    //     "John",
    //     "Doe",
    //     "1990-01-01",
    //     "123-456-7890",
    //     "johndoe",
    //     "g5H9V@example.com",
    // ).unwrap();
    // utils::archive::archive().unwrap();
    // utils::archive::archive().unwrap();
    // utils::send::send_email_with_attachment();
}
