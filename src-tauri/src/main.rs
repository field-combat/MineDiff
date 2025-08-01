/*

FileExplorerってプロジェクト。フォルダにアクセスしてるし、パクれそう
https://github.com/conaticus/FileExplorer

*/

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    minediff_lib::run();
}
