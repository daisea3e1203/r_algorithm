use self::Query::*;

enum Query {
    Add(i32, i32, i32),
    Sum(i32, i32),
}

pub fn run() {
    const N: usize = 10;
    const A: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let queries = [Sum(4, 4), Sum(1, 10), Sum(2, 4), Add(3, 6, 3), Sum(2, 4)];

    let mut bit0 = [0; N + 1];
    let mut bit1 = [0; N + 1];
    // Answer
    // 4, 55, 9, 15
    // from http://poj.org/problem?id=3468

    for i in 1..N + 1 {
        add(&mut bit0, i as i32, A[i - 1], N);
    }

    for query in queries {
        match query {
            Sum(l, r) => {
                let mut res = 0;
                res += sum(&mut bit0, r) + sum(&mut bit1, r) * r;
                res -= sum(&mut bit0, l - 1) + sum(&mut bit1, l - 1) * (l - 1);
                print!("{}\n", res);
            }
            Add(l, r, x) => {
                add(&mut bit0, l, -x * (l - 1), N);
                add(&mut bit1, l, x, N);
                add(&mut bit0, r + 1, x * r, N);
                add(&mut bit1, r + 1, -x, N);
            }
        }
    }
}

fn sum(b: &[i32], i: i32) -> i32 {
    let mut i = i;
    let mut s = 0;
    while i > 0 {
        s += b[i as usize];
        i -= i & -i;
    }
    s
}

fn add(b: &mut [i32], i: i32, v: i32, n: usize) {
    let mut i = i;
    while (i as usize) <= n {
        b[i as usize] += v;
        i += i & -i;
    }
}
