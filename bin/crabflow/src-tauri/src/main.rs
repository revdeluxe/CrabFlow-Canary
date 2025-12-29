mod sysmodules;
mod setup;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            sysmodules::config::save_setup_config,
            sysmodules::config::load_setup_config,
            sysmodules::config::reset_setup_config,
            setup::wizard::check_first_run, // ← add this line
        ])
        .run(tauri::generate_context!())
        .expect("error while running CrabFlow");
}
