use windows::{core::PSTR, Win32::{Security::SECURITY_ATTRIBUTES, System::Threading::CreateMutexA}};

#[cfg_attr(mobile, tauri::mobile_entry_point)]


pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![lock_mutex])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn lock_mutex() {
unsafe {

 let security_attributes = SECURITY_ATTRIBUTES {
   nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
   lpSecurityDescriptor: std::ptr::null_mut(),
   bInheritHandle: windows::Win32::Foundation::BOOL(0),
 };

 let mutex_name = PSTR("ROBLOX_singletonMutex".as_ptr() as _);
 let _ = CreateMutexA(Some(&security_attributes), true, mutex_name);
}
}