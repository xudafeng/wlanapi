#[macro_use]
extern crate neon;

use std::str;
use neon::prelude::*;
use neon::register_module;

#[cfg(windows)] extern crate winapi;
use std::io::Error;
use winapi::um::winnt::{HANDLE, PHANDLE, PVOID};
use std::ptr;
use winapi::um::wlanapi::{PWLAN_NOTIFICATION_DATA, WlanOpenHandle, WLAN_API_VERSION, WlanRegisterNotification, WLAN_NOTIFICATION_SOURCE_ACM};
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

#[cfg(windows)]

extern "system" fn callback(data: PWLAN_NOTIFICATION_DATA, _: PVOID) {
    f.call(data);
    println!("hello");
}

fn handle(f) {
    // https://github.com/neon-bindings/examples/blob/master/functions/native/src/lib.rs
    // https://docs.rs/winapi/0.3.9/winapi/um/wlanapi/fn.WlanRegisterNotification.html
    let client: PHANDLE = ptr::null_mut();
    let neg_verson: *mut u32 = ptr::null_mut();
    let dwPrevNotifType: *mut u32 = ptr::null_mut();
    // open client
    unsafe {
        WlanOpenHandle(WLAN_API_VERSION, ptr::null_mut(), neg_verson, client);
        WlanRegisterNotification(*client, WLAN_NOTIFICATION_SOURCE_ACM, 0, Some(callback), null_mut(), null_mut(), dwPrevNotifType);
    }
}
#[cfg(not(windows))]
fn handle() {
} 

fn registerNotification(mut cx: FunctionContext) -> JsResult<JsString> {
    handle(f);
    Ok(cx.string("hello world!"));
}

register_module!(mut cx, {
    cx.export_function("registerNotification", registerNotification)
});
