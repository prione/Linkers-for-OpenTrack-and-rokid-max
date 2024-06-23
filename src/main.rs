use std::{thread, time::Duration};

use cgmath::num_traits::ToPrimitive;
use neckswitch::{
    wrapper::{get_window_rect, load_filterd_window_handlers, move_window_pos},
    RokidMax,
};
use windows::Win32::Foundation::{HWND, RECT};

fn main() -> anyhow::Result<()> {
    let rokid_max = RokidMax::new()?;
    // センサーの値をどれだけウインドウの移動に反映させるか
    let boost = 5000.0;
    // 何ピクセル前回と差があればウインドウの移動を開始するか
    let threthold = 20.0;

    let mut rects: Vec<(HWND, RECT)> = vec![];
    let mut pre_q = rokid_max.quaternion();
    let mut pre_x_gain = 0.0;
    let mut pre_y_gain = 0.0;
    loop {
        {
            let new_handlers = load_filterd_window_handlers();
            // ウインドウが増えた場合は追加
            new_handlers.iter().for_each(|hwnd| {
                if !rects.iter().any(|(h, _)| h == hwnd) {
                    rects.push((*hwnd, get_window_rect(hwnd)));
                }
            });
            // ウインドウが減った場合は削除
            rects.retain(|(hwnd, _)| new_handlers.iter().any(|h| h == hwnd));
            // ウインドウの位置が前回のゲインと一致しない場合はユーザー操作で移動されたとみなして RECT を更新
            rects.iter_mut().for_each(|(hwnd, rect)| {
                let current_rect = get_window_rect(hwnd);
                if current_rect.left != rect.left || current_rect.top != rect.top {
                    *rect = current_rect;
                    rect.left += pre_x_gain.to_i32().unwrap();
                    rect.top += pre_y_gain.to_i32().unwrap();
                }
            });
        }

        let new_q = rokid_max.quaternion();
        let x_gain = new_q.v.y * boost;
        let y_gain = new_q.v.x * boost;
        if (x_gain - pre_x_gain).abs() > threthold || (y_gain - pre_y_gain).abs() > threthold {
            rects.iter().for_each(|(hwnd, rect)| {
                move_window_pos(
                    hwnd,
                    rect.left - x_gain.to_i32().unwrap(),
                    rect.top - y_gain.to_i32().unwrap(),
                );
            });
            pre_x_gain = x_gain;
            pre_y_gain = y_gain;
            pre_q = new_q;
        }

        thread::sleep(Duration::from_millis(10));
    }
}
