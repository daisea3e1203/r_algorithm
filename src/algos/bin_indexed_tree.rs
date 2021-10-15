fn sum(i: i32, bit: &[i32]) -> i32 {
    let mut i = i;
    let mut s = 0;
    while i > 0 {
        s += bit[(i - 1) as usize];
        i -= i & -i;
    }
    s
}

fn add(i: i32, x: i32, n: usize, bit: &mut [i32]) {
    let mut i = i;
    while i <= n as i32 {
        bit[(i - 1) as usize] += x;
        i += i & -i;
    }
}

pub fn run() {
    const N: usize = 4;
    let mut bit = [0; N];
    let a = [3, 1, 4, 2];

    let mut ans: i32 = 0;

    for j in 0..N {
        ans += j as i32 - sum(a[j], &bit);
        add(a[j], 1, N, &mut bit);
    }

    print!("{}", ans);
}
