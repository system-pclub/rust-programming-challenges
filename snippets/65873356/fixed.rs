const N: usize = 4;

fn main() {
    println!("Hello, world!");
    let bebalken = [[false; N]; N];
    let stuetzen = [[true; N]; N];


    let mut f = Feld {
        abst: [[0; N]; N],
        bebalken: &bebalken,
        stuetzen: &stuetzen,
        balken: [[0; N]; N],

    };

    f.balken_einfugen(0, 0, 1, 4, 1);
    f.balken_einfugen(1, 0, 1, 4, 1);
}


struct Feld<'a> {
    abst: [[u8; N]; N],
    bebalken: &'a [[bool; N]; N],
    stuetzen: &'a [[bool; N]; N],
    balken: [[u8; N]; N], 
  
}

impl Feld<'_> {
    pub fn balken_einfugen<'a>(&mut self, x: usize, y: usize, dir: u8, r: u8, connBar: u8) {
        let imax: usize = N;

        if dir == 1 {
            for i in 0..imax {
                self.balken[x + i][y] = connBar;
            }
        };
    }
}