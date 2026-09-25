use std::fs::File;
use std::io::Write;
use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize)]
struct Point(f64, f64);
#[derive(Serialize)]
struct Row(f64,f64,f64,f64);

fn read_from_csv(path: &str) -> Vec<Point> {
    let mut rdr = ReaderBuilder::new()
                  .has_headers(false)
                                .from_path(path).unwrap();
    rdr.records()
        .map(|r| {
            let p: Point = r.unwrap().deserialize(None).unwrap();
            p
        })
        .collect()
}

fn format_for_tex(mut a:f64, mut b:f64) -> String{
    a = (a*1000.).round() / 1000.;
    b = (b*1000.).round() / 1000.;
    match b {
        ..=0.0 => format!("y\\approx{}x{}",a,b),
        0.0.. => format!("y\\approx{}x+{}",a,b),
        _ => panic!("Unexpected")
    }
}

fn main() {
    for file in ["var4.csv", "var6.csv", "var8.csv"] {
        let points = read_from_csv(file);
        let n: f64  = points.len() as f64;
        
        let sum_x: f64 = points.iter()
                    .map(|p| p.0).sum();
        let sum_y: f64 = points.iter()
                    .map(|p| p.1).sum();
        let sum_xx: f64 = points.iter()
                    .map(|p| p.0 * p.0).sum();
        let sum_xy: f64 = points.iter()
                    .map(|p| p.0 * p.1).sum();

        let a: f64 = (n * sum_xy - sum_x * sum_y) /
                    (n * sum_xx - sum_x * sum_x);

        let b: f64 = (sum_y - a * sum_x) / n;
        
        let mut wtr = WriterBuilder::new()
                      .from_path(format!("out_{file}").as_str()).unwrap();
        let _ = wtr.serialize(Row {
            0: (sum_x*1000.).round() / 1000., 
            1: (sum_y * 1000.).round() / 1000., 
            2: (sum_xx * 1000.).round() / 1000., 
            3: (sum_xy*1000.).round() / 1000.
        });
        let _ = wtr.flush();

        let mut out_file = File::create(format!("func_{}.txt", &file[0..4])).unwrap();
        let _ = out_file.write_all(format_for_tex(a, b).as_bytes());
    }
}

