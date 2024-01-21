// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
    .setup(|app| {
        let window = app.get_window("main").unwrap();
        tauri::async_runtime::spawn(async move {
            println!("Initializing...");
            let _loader = window.eval("window.location.replace('https://www.notion.so/')");
            let css = r#"
                .notion-scroller .notion-board-view {
                    min-width: initial !important;
                }

                .notion-scroller .notion-board-view > div:not(:has(.notion-selectable-halo)) > div:last-child {
                    display: none;
                }

                .notion-scroller .notion-board-view > div:has(.notion-selectable-halo) > div:nth-last-child(2) {
                    display: none;
                }
            "#;

            let js = format!(r#"
                var style = document.createElement('style');
                style.type = 'text/css';
                style.innerHTML = `{}`;
                document.head.appendChild(style);
            "#, css);

            std::thread::sleep(std::time::Duration::from_millis(2000));
            let _invoke_first = window.eval(&js);
        });
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error running tauri app");
}