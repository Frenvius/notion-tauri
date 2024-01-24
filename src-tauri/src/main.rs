// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
    .setup(|app| {
        let window = app.get_window("main").unwrap();
        tauri::async_runtime::spawn(async move {
            println!("Initializing...");
            let _loader = window.eval("window.location.replace('https://www.notion.so/')");

            let theming_variables = read_file("src/css/theming/variables.css");
            let theming_prism = read_file("src/css/theming/prism.css");
            let theming_patches = read_file("src/css/theming/patches.css");
            let tweaks_client = read_file("src/css/tweaks/client.css");
            let nord_variables = read_file("src/css/nord/variables.css");
            let theming_theme = read_file("src/css/theming/theme.css");
            let theming_colors = read_file("src/css/theming/colors.css");

            let theming_variables_tags = create_style_tag(&theming_variables);
            let theming_prism_tags = create_style_tag(&theming_prism);
            let theming_patches_tags = create_style_tag(&theming_patches);
            let tweaks_client_tags = create_style_tag(&tweaks_client);
            let nord_variables_tags = create_style_tag(&nord_variables);
            let theming_theme_tags = create_style_tag(&theming_theme);
            let theming_colors_tags = create_style_tag(&theming_colors);

            std::thread::sleep(std::time::Duration::from_millis(2000));
            let _invoke_second = window.eval(&theming_variables_tags);
            let _invoke_third = window.eval(&theming_prism_tags);
            let _invoke_fourth = window.eval(&theming_patches_tags);
            let _invoke_fifth = window.eval(&tweaks_client_tags);
            let _invoke_sixth = window.eval(&nord_variables_tags);
            let _invoke_seventh = window.eval(&theming_theme_tags);
            let _invoke_eighth = window.eval(&theming_colors_tags);

            let js = read_file("src/js/scripts.js");
            let _invoke_ninth = window.eval(&js);
        });
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error running tauri app");
}

fn create_style_tag(css: &str) -> String {
    format!(r#"
        var style = document.createElement('style');
        style.type = 'text/css';
        style.innerHTML = `{}`;
        document.head.appendChild(style);
    "#, css)
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect(&format!("Unable to read file {}", path))
}