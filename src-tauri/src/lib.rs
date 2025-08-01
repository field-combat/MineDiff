/*
フォルダ選択ってどうやるねん
TauriのAPI叩く感じっぽい
https://zenn.dev/upopon/articles/7c610d86c9eae0
https://dj-dz.hatenablog.com/entry/2023/02/26/170531

一旦、フォルダパスはコピペで持ってくるようにする
*/

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
// use std::process::Command;
// use once_cell::sync::OnceCell;
// use tauri::api::dialog;
// static folder_name: OnceCell<String> = OnceCell::new();
// use tauri::api::dialog::FileDialogBuilder;
// use tauri::api::dialog::blocking::FileDialogBuilder;
// mod commands;
// use commands::{cmd, invoke, message, resolver};
// use std::path::PathBuf;
// use tauri::{AppHandle, Manager};

/* // 元コード（壊れたらこっちを使う）
fn greet(name: &str) -> String {
    println!("Backend was called with an argument: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}
// */
//*/ #[must_use]
#[warn(unused_imports)]
// fn greet(name: &str) -> String {
fn greet(_name: &str, _input_f: &str, _output_f: &str, _dur: &str) {
    // fn greet(name: &str, _input_f: &str, _output_f: &str, _dur: &str) -> String {
    use std::process::Command;
    println!("test");
    // println!("Backend was called with an argument: {}", _input_f);
    // format!("Hello, {}! You've been greeted from Rust!", name)

    //*
    // use std::process::Command;
    // format!("Hello, {}! You've been greeted from Rust!", _name);

    // 日付フォーマット変更
    let prefix: String = String::from("/MAXAGE:");
    let d = _dur.replace("-", "");
    let d1 = prefix + &d;

    let d2 = d1.as_str();

    // ファイルパス
    let in_f = _input_f.replace("\"", "");
    let out_f = _output_f.replace("\"", "");

    // サブディレクトリオプション
    let opt = String::from("/S");

    let _output = Command::new("robocopy")
        .args(&[in_f, out_f, opt, d1])
        .output()
        .expect("failed to start `ls`");
    println!("{}", String::from_utf8_lossy(&_output.stdout));
    // return true;
    // */

    // println!("{}", String::from_utf8_lossy(&_output.stdout))
}
// */
// fn greet(name: &str) -> String {
//     // fn greet(name: &str, inputF: &str, outputF: &str, dur: &str) -> String {
//     // fn greet(name: &str, inputF: &str, outputF: &str, dur: &str) {
//     format!("Hello, {}! You've been greeted from Rust!", name);
//     // format!("input, {}! You've been greeted from Rust!", inputF);
//     // format!("output, {}! You've been greeted from Rust!", outputF);
//     // format!("date, {}! You've been greeted from Rust!", dur)
// }

// // 入力フォルダ選択
// fn select_folder() -> Option<String> {
//     let (sender, receiver) = std::sync::mpsc::channel();
//     FileDialogBuilder::new().pick_folder(move |folder_path| {
//         if let Some(path) = folder_path {
//             sender.send(path.to_string_lossy().to_string()).unwrap();
//         } else {
//             sender.send(String::new()).unwrap();
//         }
//     });
//     receiver.recv().ok()
// }
// // fn selectInput() {
// //     // dialog::FileDialogBuilder::default().pick_folder(|path_buf| match path_buf {
// //     //     Some(p) => {
// //     //         folder_name.set(p.into_os_string().into_string().unwrap());

// //     //         println!("選択されたフォルダ: {}", folder_name.get().unwrap());
// //     //     }

// //     //     _ => {}
// //     // });
// // }

// // 出力先フォルダ選択
// fn selectOutput() {
//     // dialog::FileDialogBuilder::default().pick_folder(|path_buf| match path_buf {
//     //     Some(p) => {
//     //         folder_name.set(p.into_os_string().into_string().unwrap());

//     //         println!("選択されたフォルダ: {}", folder_name.get().unwrap());
//     //     }

//     //     _ => {}
//     // });
// }

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![greet, selectInput, selectOutput])
        .invoke_handler(tauri::generate_handler![greet])
        // .invoke_handler(tauri::generate_handler![select_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
