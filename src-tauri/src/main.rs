// MedFlow CRM — настольная оболочка (Tauri v2).
// Программа открывает рабочий экран CRM (https://leclinic-ryazan.ru/crm/)
// в отдельном окне как обычное настольное приложение. Сервер (WordPress)
// должен быть доступен по сети — оболочка только показывает его.
//
// Чтобы поменять адрес: замените строку CRM_URL ниже и пересоберите.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

// >>> ЕДИНСТВЕННОЕ, ЧТО МЕНЯЕТСЯ ПРИ СМЕНЕ САЙТА <<<
const CRM_URL: &str = "https://leclinic-ryazan.ru/crm/";
const WINDOW_TITLE: &str = "MedFlow CRM — ЛЕ Клиника";

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let url = CRM_URL
                .parse()
                .expect("CRM_URL должен быть корректным адресом https://");

            let win = WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
                .title(WINDOW_TITLE)
                .inner_size(1360.0, 900.0)
                .min_inner_size(1024.0, 680.0)
                .center()
                .resizable(true)
                .maximizable(true)
                .build()?;

            // Если сайт недоступен (нет сети/сервер выключен), окно всё равно
            // откроется — WebView покажет свою страницу ошибки. Это ожидаемо:
            // оболочка не хранит данные, она лишь показывает серверный /crm/.
            let _ = win.set_focus();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("не удалось запустить MedFlow CRM");
}
