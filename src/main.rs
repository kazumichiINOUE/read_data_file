use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    // データファイルのオープン
    let log_path = "log_data";
    let file = File::open(Path::new(&log_path)).expect("ログファイルのオープンに失敗しました");
    let reader = BufReader::new(file);

    // 読み込んだデータを格納するベクトル
    let mut points_data = Vec::<(f64, f64, f64)>::new();

    // ファイルからデータを読み込み，ベクトルへ格納
    for line in reader.lines() {
        if let Ok(line) = line {
            // 行を空白で分割
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() == 7 {   // 7はlod_dataの列数
                let t: i64 = parts[0].parse().expect("Tのパースに失敗しました");
                let x: f64 = parts[1].parse().expect("Xのパースに失敗しました");
                let y: f64 = parts[2].parse().expect("Yのパースに失敗しました");
                let a: f64 = parts[3].parse().expect("Aのパースに失敗しました");
                let n: f64 = parts[4].parse().expect("NUMのパースに失敗しました");
                let f: i32 = parts[5].parse().expect("FLAGのパースに失敗しました");
                let s: String = parts[6].parse().expect("endのパースに失敗しました");
                points_data.push((x, y, a));
            }
        }
    }

    for data in points_data {
        println!("x:{} y:{} a:{}", data.0, data.1, data.2);
    }
}


