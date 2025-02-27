use crate::AppleCalendarPluginExt;

#[tauri::command]
#[specta::specta]
pub fn open_calendar_access_settings<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    app.open_calendar_access_settings()
}

#[tauri::command]
#[specta::specta]
pub fn open_contacts_access_settings<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    app.open_contacts_access_settings()
}

#[tauri::command]
#[specta::specta]
pub fn calendar_access_status<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> bool {
    app.calendar_access_status()
}

#[tauri::command]
#[specta::specta]
pub fn contacts_access_status<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> bool {
    app.contacts_access_status()
}

#[tauri::command]
#[specta::specta]
pub fn request_calendar_access<R: tauri::Runtime>(app: tauri::AppHandle<R>) {
    app.request_calendar_access();
}

#[tauri::command]
#[specta::specta]
pub fn request_contacts_access<R: tauri::Runtime>(app: tauri::AppHandle<R>) {
    app.request_contacts_access();
}

#[tauri::command]
#[specta::specta]
pub fn start_worker<R: tauri::Runtime>(app: tauri::AppHandle<R>, user_id: String) {
    app.start_worker(user_id).unwrap();
}

#[tauri::command]
#[specta::specta]
pub fn stop_worker<R: tauri::Runtime>(app: tauri::AppHandle<R>) {
    app.stop_worker();
}
