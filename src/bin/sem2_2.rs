use std::ops::Sub;
use csv::WriterBuilder;
use serde::Serialize;

#[derive(Clone, Copy,Serialize)]
struct Row{
    n: usize,
    x: f64,
    y: f64,
    z: f64
}

fn x(y:f64, z:f64) -> f64 {
    4. - 3. * y + 2. * z 
}

fn y(x: f64, z: f64) -> f64 {
    1.4 - 0.4 * x  - 0.2 * z
}

fn z(x: f64, y:f64) -> f64 {
    0.25 - 0.125 * x - 0.125 * y
}

impl Sub for Row {
    type Output = Row;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            n: 0,
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Row {
    fn norma(&self) -> f64 {
        self.x.max(self.y).max(self.z)
    }
}


fn main() {
    let eps: f64 = 0.1;
    let q: f64 = -0.25;
    let mut X: Vec<Row> = vec![Row {n: 0, x: 4., y: 1.4, z: 0.25}];
    let mut i: usize = 0;
    let right_side = eps * (1.-q) / q;

    loop {
        let xn = x(X[i].y, X[i].z);
        let yn = y(X[i].x, X[i].z);
        let zn = z(X[i].x, X[i].y);
        i += 1;

        X.push(Row { n: i, x: xn, y: yn, z: zn });
        let norm = (X[i] - X[i-1]).norma();
        if norm <= right_side{
            break;
        }
    }
    
    let mut wtr =  WriterBuilder::new()
                   .has_headers(true)
                   .from_path("output_iterations.csv").unwrap();
    for i in X {
        let _ = wtr.serialize(i);
        let _ = wtr.flush();
    }
            
}