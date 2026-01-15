// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Listener, Manager, WindowEvent, WebviewWindowBuilder, WebviewUrl,
};
use tauri_plugin_autostart::MacosLauncher;

// JavaScript to inject for notification support and performance optimization
const INIT_SCRIPT: &str = r#"
(function() {
    'use strict';
    
    // ===== NOTIFICATION SUPPORT =====
    const OriginalNotification = window.Notification;
    
    Object.defineProperty(window.Notification, 'permission', {
        get: () => 'granted',
        configurable: true
    });
    
    window.Notification.requestPermission = (callback) => {
        if (callback) callback('granted');
        return Promise.resolve('granted');
    };
    
    window.Notification = function(title, options = {}) {
        if (window.__TAURI__?.event) {
            window.__TAURI__.event.emit('show-notification', {
                title: title,
                body: options.body || ''
            });
        }
        try { return new OriginalNotification(title, options); } catch(e) {}
    };
    window.Notification.permission = 'granted';
    window.Notification.requestPermission = (cb) => {
        if (cb) cb('granted');
        return Promise.resolve('granted');
    };
    
    // ===== PERFORMANCE OPTIMIZATIONS =====
    // Reduce animation frame rate when hidden
    let _raf = window.requestAnimationFrame;
    let _isVisible = true;
    
    document.addEventListener('visibilitychange', () => {
        _isVisible = !document.hidden;
    });
    
    // Throttle RAF when window is hidden to save CPU
    window.requestAnimationFrame = function(callback) {
        if (!_isVisible) {
            return setTimeout(callback, 100);
        }
        return _raf.call(window, callback);
    };
    
    // Disable some heavy features for performance
    if (navigator.serviceWorker) {
        // Let service workers work normally for offline support
    }
    
    console.log('%c WhatsApp Desktop v1.1.0 ', 'background: #25D366; color: white; font-size: 14px; padding: 5px;');
    console.log('%c Optimized for performance ', 'color: #888;');
})();
"#;

fn main() {
    // Check if started with --autostart flag (minimized to tray)
    let start_minimized = std::env::args().any(|arg| arg == "--autostart");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--autostart"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // Focus the main window when trying to open another instance
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .setup(move |app| {
            // Setup Windows startup with proper app name
            #[cfg(target_os = "windows")]
            {
                setup_windows_autostart();
            }
            
            let window = app.get_webview_window("main").unwrap();
            
            // If started with --autostart, hide window (start in tray)
            if start_minimized {
                let _ = window.hide();
            }
            
            // Create tray menu
            let show_item = MenuItem::with_id(app, "show", "Show WhatsApp", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            let separator = MenuItem::with_id(app, "sep", "─────────", false, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            
            let menu = Menu::with_items(app, &[&show_item, &hide_item, &separator, &quit_item])?;
            
            // Build tray icon
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .tooltip("WhatsApp Desktop")
                .on_menu_event(move |app, event| {
                    if let Some(window) = app.get_webview_window("main") {
                        match event.id.as_ref() {
                            "show" => {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                            "hide" => {
                                let _ = window.hide();
                            }
                            "quit" => {
                                std::process::exit(0);
                            }
                            _ => {}
                        }
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            // Handle window close - minimize to tray instead
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window_clone.hide();
                }
            });

            // Inject notification script when page loads
            let window_for_script = app.get_webview_window("main").unwrap();
            let app_handle = app.handle().clone();
            
            // Listen for notification events from JavaScript
            app.listen("show-notification", move |event: tauri::Event| {
                if let Ok(payload) = serde_json::from_str::<serde_json::Value>(event.payload()) {
                    let title = payload.get("title").and_then(|v| v.as_str()).unwrap_or("WhatsApp");
                    let body = payload.get("body").and_then(|v| v.as_str()).unwrap_or("");
                    
                    // Show native notification
                    let _ = app_handle.emit("tauri://notification", serde_json::json!({
                        "title": title,
                        "body": body
                    }));
                }
            });
            
            // Inject the script faster using page load event
            let window_for_script = app.get_webview_window("main").unwrap();
            std::thread::spawn(move || {
                // Wait for page to be ready
                std::thread::sleep(std::time::Duration::from_millis(1500));
                let _ = window_for_script.eval(INIT_SCRIPT);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            toggle_autostart,
            get_autostart_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn toggle_autostart(app: tauri::AppHandle, enable: bool) -> Result<bool, String> {
    use tauri_plugin_autostart::ManagerExt;
    
    let autostart_manager = app.autolaunch();
    
    if enable {
        autostart_manager.enable().map_err(|e| e.to_string())?;
    } else {
        autostart_manager.disable().map_err(|e| e.to_string())?;
    }
    
    Ok(enable)
}

#[tauri::command]
async fn get_autostart_status(app: tauri::AppHandle) -> Result<bool, String> {
    use tauri_plugin_autostart::ManagerExt;
    
    let autostart_manager = app.autolaunch();
    autostart_manager.is_enabled().map_err(|e| e.to_string())
}

#[cfg(target_os = "windows")]
fn setup_windows_autostart() {
    use std::process::Command;
    #[cfg(target_os = "windows")]
    use std::os::windows::process::CommandExt;
    
    // Get the current executable path
    if let Ok(exe_path) = std::env::current_exe() {
        let exe_str = exe_path.to_string_lossy();
        
        // Create registry entry with proper app name using PowerShell
        // This creates a proper startup entry that shows "WhatsApp Desktop" in Task Manager
        let ps_script = format!(
            r#"
            $regPath = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Run'
            $appName = 'WhatsApp Desktop'
            $appPath = '"{}" --autostart'
            
            # Remove old generic entry if exists
            Remove-ItemProperty -Path $regPath -Name 'whatsapp-desktop' -ErrorAction SilentlyContinue
            Remove-ItemProperty -Path $regPath -Name 'Program' -ErrorAction SilentlyContinue
            
            # Add proper named entry
            Set-ItemProperty -Path $regPath -Name $appName -Value $appPath
            "#,
            exe_str
        );
        
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let _ = Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", &ps_script])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
    }
}
