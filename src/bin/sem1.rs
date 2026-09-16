fn f(x: f64, y: f64) -> f64 {
    x.cos() - y
}

#[derive(Copy, Clone, Debug)]
struct Row {
    x: f64,
    y: f64,
    k1: f64,
    k2: f64,
    k3: f64,
    k4: f64,
}

impl Row {
    pub fn kn(&self) -> f64 {
        self.k1 + 2. * self.k2 + 2. * self.k3 + self.k4
    }
    pub fn new() -> Self {
        Self {
            x: 0.,
            y: 0.,
            k1: 0.,
            k2: 0.,
            k3: 0.,
            k4: 0.,
        }
    }
}

fn main() {
    let h: f64 = 0.01;
    let mut table: [Row; 11] = [Row::new(); 11];
    table[0].x = 0.;
    table[0].y = 0.5;
    for i in 0..10 {
        table[i].k1 = f(table[i].x, table[i].y);
        table[i].k2 = f(table[i].x + h / 2., table[i].y + h / 2. * table[i].k1);
        table[i].k3 = f(table[i].x + h / 2., table[i].y + h / 2. * table[i].k2);
        table[i].k4 = f(table[i].x + h, table[i].y + h * table[i].k3);

        table[i + 1].x = table[i].x + h;
        table[i + 1].y = table[i].y + h * (1. / 6.) * table[i].kn();
    }
    for row in table {
        println!("{:?}", row);
    }
}