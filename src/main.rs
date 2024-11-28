use anyhow::Result;
use crate::lib::RokidMax;
use cgmath::{Quaternion, Euler};
use std::net::UdpSocket;
use std::time::Duration;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Cursor;

mod lib;

fn main() -> Result<()> {
    // RokidMaxを初期化
    let device = RokidMax::new()?;

    // UDPソケットの設定
    let target = "127.0.0.1:4242"; // OpenTrackがリッスンしているポート
    let socket = UdpSocket::bind("127.0.0.1:5000")?; // 任意のローカルポートをバインド
    socket.set_read_timeout(Some(Duration::new(1, 0)))?; // 1秒のタイムアウト設定

    loop {
        let quat = device.quaternion(); // クォータニオンを取得
        let (pitch, roll, yaw) = quaternion_to_euler(&quat);

        // 仮の位置データとして (x, y, z) を設定
        let x = 0.0;
        let y = 0.0;
        let z = 0.0;

        // 6つの倍精度浮動小数点数 (x, y, z, pitch, roll, yaw) をリトルエンディアンでパック
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);

        // 位置データ
        cursor.write_f64::<LittleEndian>(x)?;
        cursor.write_f64::<LittleEndian>(y)?;
        cursor.write_f64::<LittleEndian>(z)?;

        // 回転データ (オイラー角: 度)
        cursor.write_f64::<LittleEndian>(yaw)?; // ヨー (degree)
        cursor.write_f64::<LittleEndian>(pitch)?; // ピッチ (degree)
        cursor.write_f64::<LittleEndian>(roll)?; // ロール (degree)

        // OpenTrackに送信するデータを送信
        socket.send_to(&buf, target)?;

        // 5msごとに更新
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

// クォータニオンからオイラー角 (ピッチ, ロール, ヨー) を計算する関数
fn quaternion_to_euler(quat: &Quaternion<f32>) -> (f64, f64, f64) {
    let euler_angles = Euler::from(*quat); // クォータニオンからオイラー角に変換

    // ラジアンを度に変換
    let rad_to_deg = 180.0 / std::f64::consts::PI;
    (
        (euler_angles.x.0 as f64) * rad_to_deg, // ピッチ (degree)
        (euler_angles.y.0 as f64) * rad_to_deg, // ロール (degree)
        (euler_angles.z.0 as f64) * rad_to_deg, // ヨー (degree)
    )
}
