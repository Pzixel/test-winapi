mod bindings {
    ::windows::include_bindings!();
}

use bindings::{
    windows::win32::system_services::{
        CreateEventW, SetEvent, WaitForSingleObject, PWSTR, WAIT_RETURN_CAUSE,
    },
    windows::win32::windows_programming::CloseHandle,
    windows::win32::windows_and_messaging::{MessageBoxA, HWND, MB_FLAGS},
};

fn main() -> windows::Result<()> {
    unsafe {
        let event = CreateEven3tW(std::ptr::null_mut(), true, false, PWSTR::default());

        assert!(event.0 != 0);

        SetEvent(event).ok()?;

        let result = WaitForSingleObject(event, 0);
        assert!(result == WAIT_RETURN_CAUSE::WAIT_OBJECT_0);

        MessageBoxA(HWND(0), "Text", "Caption", MB_FLAGS::MB_OK);

        CloseHandle(event).ok()
    }
}