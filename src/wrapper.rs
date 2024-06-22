use std::{
    ffi::c_void,
    mem::{self, size_of_val},
};

use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM, RECT},
    Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_CLOAKED},
    UI::WindowsAndMessaging::{
        EnumWindows, GetWindowRect, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible,
        SetWindowPos, ShowWindow, SWP_NOSIZE, SWP_NOZORDER, SW_RESTORE,
    },
};

pub fn get_window_rect(hwnd: &HWND) -> RECT {
    unsafe {
        let window_rect = &mut RECT::default();
        let _ = GetWindowRect(*hwnd, window_rect);
        *window_rect
    }
}

pub fn move_window_pos(hwnd: &HWND, x: i32, y: i32) {
    unsafe {
        let _ = ShowWindow(*hwnd, SW_RESTORE);
        // 第二引数は API の本来的な意味でも HWND 型は適切ではない気がするが、0 で問題ない(HWND_TOP を指定している)
        // https://learn.microsoft.com/ja-jp/windows/win32/api/winuser/nf-winuser-setwindowpos
        let _ = SetWindowPos(*hwnd, HWND(0), x, y, 0, 0, SWP_NOSIZE | SWP_NOZORDER);
    }
}

pub fn load_filterd_window_handlers() -> Vec<HWND> {
    load_window_handlers()
        .into_iter()
        .filter(is_visible)
        .filter(|hwnd| !is_cloacked(hwnd))
        .filter(|hwnd| !is_ignore_window_title(hwnd))
        .collect()
}

pub(crate) fn load_window_handlers() -> Vec<HWND> {
    unsafe {
        let handlers: Vec<HWND> = Vec::new();
        // handlers は Vec なので、そのポインタを取得するために as *const _ を使う
        let _ = EnumWindows(
            Some(self::enum_callback),
            LPARAM(&handlers as *const _ as _),
        );
        handlers
    }
}

unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let handlers: &mut Vec<HWND> = mem::transmute(lparam);
    handlers.push(hwnd);
    BOOL::from(true)
}

fn is_visible(hwnd: &HWND) -> bool {
    unsafe { IsWindowVisible(*hwnd).as_bool() }
}

fn is_cloacked(hwnd: &HWND) -> bool {
    unsafe {
        let mut cloacked = vec![0_u32];

        let result = DwmGetWindowAttribute(
            *hwnd,
            DWMWA_CLOAKED,
            cloacked.as_mut_ptr() as *mut c_void,
            size_of_val(&cloacked) as u32,
        );

        if result.is_err() {
            return false;
        }

        // https://docs.microsoft.com/en-us/windows/win32/api/dwmapi/ne-dwmapi-dwmwindowattribute
        return if let Some(result) = cloacked.first() {
            *result != 0_u32
        } else {
            false
        };
    }
}

fn is_ignore_window_title(hwnd: &HWND) -> bool {
    matches!(
        get_window_title(hwnd).as_deref(),
        Some("Program Manager")
            | Some("ZPToolBarParentWnd")
            | Some("ConfMeetingNotfiyWnd")
            | Some("画面共有表示オプション")
            | Some("Enpass")
            | Some("セットアップ")
            | Some("ミーティングコントロール")
            | Some("VideoFrameWnd")
            | Some("ScreenToGif")
            | None
    )
}

pub(crate) fn get_window_title(hwnd: &HWND) -> Option<String> {
    unsafe {
        let title_text_length = GetWindowTextLengthW(*hwnd);
        if title_text_length == 0 {
            return None;
        }
        let mut buffer: [u16; 512] = [0; 512];
        let len = GetWindowTextW(*hwnd, &mut buffer);
        let widestr = String::from_utf16_lossy(&buffer[..len as usize]);
        Some(widestr)
    }
}
