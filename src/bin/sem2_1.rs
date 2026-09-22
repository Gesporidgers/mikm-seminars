use csv::{ReaderBuilder};
use serde::Deserialize;

#[derive(Debug,Deserialize)]
struct Point(f64, f64);

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

fn format_for_tex(a:f64, b:f64) -> String{
    a = (a*1000.).round() / 1000.;
    b = (b*1000.).round() / 1000.;
    
}

fn main() {
    let mut points = read_from_csv("var4.csv");
    
    println!("{:?}", points);
}

