use std::f64::consts::PI;

const ST_SIZE: usize = (1 << 14) - 1;
const MAX_N: usize = 10000;

// Initialize Segment Tree
fn init(
    k: usize,
    l: usize,
    r: usize,
    vx: &mut [f64],
    vy: &mut [f64],
    // !!! ls in the original code
    ls: &mut [i32],
    ang: &mut [f64],
) {
    ang[k] = 0.0;
    vx[k] = 0.0;

    if r - l == 1 {
        // lseaf
        vy[k] = ls[l] as f64;
    } else {
        // Range
        let chl = k * 2 + 1;
        let chr = k * 2 + 2;
        init(chl, l, (l + r) / 2, vx, vy, ls, ang);
        init(chr, (l + r) / 2, r, vx, vy, ls, ang);
        vy[k] = vy[chl] + vy[chr];
    }
}

// Rotate s by a.
fn change(
    s: usize,
    a: f64,
    v: usize,
    l: usize,
    r: usize,
    vx: &mut [f64],
    vy: &mut [f64],
    ang: &mut [f64],
) {
    if s <= l {
        return;
    } else if s < r {
        let chl = v * 2 + 1;
        let chr = v * 2 + 2;
        let m = (l + r) / 2;
        change(s, a, chl, l, m, vx, vy, ang);
        change(s, a, chr, m, r, vx, vy, ang);
        // Add to ang if the left child includes the rotated segment s
        if s <= m {
            ang[v] += a;
        }
        // Update vectors
        let s = ang[v].sin();
        let c = ang[v].cos();
        vx[v] = vx[chl] + (c * vx[chr] - s * vy[chr]);
        vy[v] = vy[chl] + (s * vx[chr] + c * vy[chr]);
    }
}

pub fn run() {
    // Inputs
    // let [mut N, mut C] = [0; 2];
    const N: usize = 3;
    const C: usize = 2;
    // let mut ls = [0; MAX_N];
    let mut ls = [5, 5, 5];
    // let [mut S, mut A] = [[0; MAX_C]; 2];
    const S: [usize; 2] = [1, 2];
    const A: [usize; 2] = [270, 90];

    let [mut vx, mut vy, mut ang] = [[0.0; ST_SIZE]; 3];
    let mut prv = [0.0; MAX_N];

    init(0, 0, N, &mut vx, &mut vy, &mut ls, &mut ang);

    for i in 1..N {
        prv[i] = PI;
    }

    for i in 0..C {
        let s = S[i];
        let a = A[i] as f64 / 360.0 * 2.0 * PI;

        change(s, a - prv[s], 0, 0, N, &mut vx, &mut vy, &mut ang);
        prv[s] = a;

        print!("{:.2} {:.2}\n", vx[0], vy[0]);
    }
}
