// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use folder_counter::path_num::index::FileNum;
use slint::SharedString;
use slint_rust_template::pathNum::index::FileNum;
use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            // ui.set_counter(ui.get_counter() + 1);
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                // self.picked_path = Some(path.display().to_string());
                println!("{:?}", path);
                let fix = ui.get_counter();
                println!("{:?}", fix);
                // ui.
                ui.set_fileDir(SharedString::from(path.display().to_string()));
                // ui.set_button_visible(false);
                // ui.se
                // println!("{:?}", ui.);
                // ui.set_file_dir(path.display().to_string());
            }
        }
    });
    ui.on_request_file_dir({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let fix = ui.get_counter();
            let path = ui.get_fileDir();
            println!("{:?}  --  {:?}", fix, path);
            let num = FileNum::from(vec![path.to_string()]);
            ui.set_counter(num as i32);
        }
    });

    ui.run()?;

    Ok(())
}
