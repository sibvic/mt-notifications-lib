pub mod server;
use server::PRConnection;
use widestring::{U16CString};
use std::sync::{Mutex, Once};
use std::{mem::MaybeUninit};

struct SingletonReader {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    inner: Mutex<PRConnection>,
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn simple() {
//         let key_str = String::from("1");
//         let text_str = String::from("2");
//         let instrument_str = String::from("3");
//         let timeframe_str = String::from("4");
//         let mut data = get_connection().inner.lock().unwrap();
//         const LOCALHOST_DOMAIN_HTTP: &str = "http://127.0.0.1:55127";
//         data.send_alert(&key_str, &text_str, &instrument_str, &timeframe_str, &String::from(LOCALHOST_DOMAIN_HTTP));
//     }
// }

fn get_connection() -> &'static SingletonReader {
    // Create an uninitialized static
    static mut SINGLETON: MaybeUninit<SingletonReader> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = SingletonReader {
                inner: Mutex::new(PRConnection::new()),
            };
            // Store it to the static var, i.e. initialize it
            SINGLETON.write(singleton);
        });

        // Now we give out a shared reference to the data, which is safe to use
        // concurrently.
        SINGLETON.assume_init_ref()
    }
}

const PROFIT_ROBOTS_DOMAIN_HTTP: &str = "https://profitrobots.com";
const SERVER_TYPE_PROFIT_ROBOTS: i8 = 0;

#[no_mangle]
extern "C" fn AdvancedAlert(key: *const u16, text: *const u16, instrument: *const u16, timeframe: *const u16) {
    let key_str = unsafe { U16CString::from_ptr_str(key).to_string().unwrap() };
    let text_str = unsafe { U16CString::from_ptr_str(text).to_string().unwrap() };
    let instrument_str = unsafe { U16CString::from_ptr_str(instrument).to_string().unwrap() };
    let timeframe_str = unsafe { U16CString::from_ptr_str(timeframe).to_string().unwrap() };
    let mut data: std::sync::MutexGuard<'_, PRConnection> = get_connection().inner.lock().unwrap();
    data.send_alert(&key_str, &text_str, &instrument_str, &timeframe_str, &String::from(PROFIT_ROBOTS_DOMAIN_HTTP));
}

#[no_mangle]
extern "C" fn AdvancedAlertCustom(key: *const u16, text: *const u16, instrument: *const u16, timeframe: *const u16, url: *const u16)
{
    let key_str = unsafe { U16CString::from_ptr_str(key).to_string().unwrap() };
    let text_str = unsafe { U16CString::from_ptr_str(text).to_string().unwrap() };
    let instrument_str = unsafe { U16CString::from_ptr_str(instrument).to_string().unwrap() };
    let timeframe_str = unsafe { U16CString::from_ptr_str(timeframe).to_string().unwrap() };
    let url_str = unsafe { U16CString::from_ptr_str(url).to_string().unwrap() };
    let mut data = get_connection().inner.lock().unwrap();
    data.send_alert(&key_str, &text_str, &instrument_str, &timeframe_str, &url_str);
}

#[no_mangle]
extern "C" fn GetServer(server_type: i8) -> *const u16
{
    let server = match server_type {
        SERVER_TYPE_PROFIT_ROBOTS => String::from(PROFIT_ROBOTS_DOMAIN_HTTP),
        _ => String::from(PROFIT_ROBOTS_DOMAIN_HTTP)
    };
    static mut LAST_MESSAGE: Option<U16CString> = None;
    unsafe {
        LAST_MESSAGE = Some(U16CString::from_str(server).unwrap());
        match &LAST_MESSAGE {
            None => return std::ptr::null(),
            Some(s) => return s.as_ptr()
        }
    }
}
