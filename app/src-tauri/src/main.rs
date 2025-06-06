// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod daemon;
mod env;

fn main() {
    qino_feed_client_lib::run()
}
