use std::fs;
use std::io::Write;
use std::path::Path;

mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[tauri::command]
fn data(
    mail: &str,
    phone: &str,
    userName: &str,
    userFname: &str,
    userLname: &str,
    birthDate: &str,
) -> String {
    let path = "/Users/eboniequ/files/data.txt";

    if Path::new(path).exists() {
        fs::remove_file(path).unwrap();
    }
    utils::sqlite::createdb().unwrap();
    utils::post::insert_data(
        userFname,
        userLname,
        birthDate,
        phone,
        userName,
        mail,
    )
    .unwrap();
    let mut output = fs::File::create(path).unwrap();
    writeln!(output, "email: {}", mail).unwrap();
    writeln!(output, "phone: {}", phone).unwrap();
    writeln!(output, "username: {}", userName).unwrap();
    writeln!(output, "first name: {}", userFname).unwrap();
    writeln!(output, "last name: {}", userLname).unwrap();
    writeln!(output, "birthday: {}", birthDate).unwrap();
    output.flush().unwrap();
    println!("ok");
    utils::archive::archive().unwrap();
    utils::send::open_mail_with_attachment();
    format!("Data saved ({},{},{},{},{},{})", mail, phone, userName, userFname, userLname, birthDate)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    // let devtools = tauri_plugin_devtools::init();
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
