use std::{thread, time::Duration};

use cgmath::num_traits::ToPrimitive;
use neckswitch::{
    window_titles,
    wrapper::{get_window_rect, load_filterd_window_handlers, move_window_pos},
    RokidMax,
};
use windows::Win32::Foundation::{HWND, RECT};

fn main() -> anyhow::Result<()> {
    let rokid_max = RokidMax::new()?;

    let handlers = load_filterd_window_handlers();
    let rects: Vec<(HWND, RECT)> = handlers
        .iter()
        .map(|hwnd| (*hwnd, get_window_rect(hwnd)))
        .collect();
    let mut pre_q = rokid_max.quaternion();
    loop {
        // 現状だとウインドウがぐらぐら動きすぎるのでもう少し安定させたい
        let new_q = rokid_max.quaternion();
        rects.iter().for_each(|(hwnd, rect)| {
            move_window_pos(
                hwnd,
                rect.left - (new_q.v.y * 5000.0).to_i32().unwrap(),
                rect.top - (new_q.v.x * 5000.0).to_i32().unwrap(),
            );
        });
        pre_q = new_q;
        thread::yield_now();
    }
}
