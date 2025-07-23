pub fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn lcm_rang(a: usize, b: usize) -> usize {
    let mut l = 1;
    let a = if a > 1 { a } else { 1 };
    for x in a..=b {
        l = lcm(x, l);
    }

    l
}
