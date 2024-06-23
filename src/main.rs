use std::{thread, time::Duration};

use cgmath::num_traits::ToPrimitive;
use neckswitch::{
    wrapper::{get_window_rect, load_filterd_window_handlers, move_window_pos},
    RokidMax,
};
use nenobi::array::TimeBaseEasingValueN;
use windows::Win32::Foundation::{HWND, RECT};

fn main() -> anyhow::Result<()> {
    let rokid_max = RokidMax::new()?;
    // センサーの値をどれだけウインドウの移動に反映させるか
    let boost = 5000.0;
    // 何ピクセル前回と差があればウインドウの移動を開始するか
    let threthold = 100.0;

    let mut rects: Vec<(HWND, RECT)> = vec![];
    let mut xy_gain = TimeBaseEasingValueN::new([0.0, 0.0]);
    let mut pre_xy_gain = [0.0, 0.0];
    loop {
        let [last_x_gain, last_y_gain] = xy_gain.last_value();
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
                    rect.left += pre_xy_gain[0].to_i32().unwrap();
                    rect.top += pre_xy_gain[1].to_i32().unwrap();
                }
            });
        }

        let new_q = rokid_max.quaternion();
        let x_gain = new_q.v.y * boost;
        let y_gain = new_q.v.x * boost;
        if (x_gain - last_x_gain).abs() > threthold || (y_gain - last_y_gain).abs() > threthold {
            xy_gain.update(
                [x_gain, y_gain],
                Duration::from_millis(500),
                nenobi::functions::quad_in_out,
            );
        }
        {
            let [current_x_gain, current_y_gain] = xy_gain.current_value();
            rects.iter().for_each(|(hwnd, rect)| {
                move_window_pos(
                    hwnd,
                    rect.left - current_x_gain.to_i32().unwrap(),
                    rect.top - current_y_gain.to_i32().unwrap(),
                );
            });
            pre_xy_gain = [current_x_gain, current_y_gain];
        }

        thread::sleep(Duration::from_millis(10));
    }
}
