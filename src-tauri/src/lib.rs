mod api;

use api::gh_request;

use std::{thread, time};

use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_http::reqwest;
use tauri_plugin_shell::ShellExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            // Autostart
            app.handle()
                .plugin(tauri_plugin_autostart::init(
                    MacosLauncher::LaunchAgent,
                    None,
                ))
                .unwrap();
            let _ = app.autolaunch().enable();

            // HTTP client
            let client = reqwest::Client::new();

            // Tray icon setup
            let quit = MenuItemBuilder::new("Quit").id("quit").build(app).unwrap();
            let menu = MenuBuilder::new(app).items(&[&quit]).build().unwrap();
            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        let url = "https://github.com/notifications";
                        app.shell().open(url, None).unwrap();
                    }
                    _ => {}
                })
                .title("0")
                .build(app)?;

            // Loop to check for notifications
            tauri::async_runtime::spawn(async move {
                loop {
                    let notifications_num = gh_request(&client).await.unwrap();
                    let title_option = Some(notifications_num.to_string());
                    tray.set_title(title_option).unwrap();

                    // sleep for 5 minutes
                    thread::sleep(time::Duration::from_secs(60 * 5));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
