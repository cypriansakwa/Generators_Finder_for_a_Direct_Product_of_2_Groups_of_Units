use std::collections::HashSet;

struct ZnStarZmStar {
    n: u32,
    m: u32,
}

impl ZnStarZmStar {
    fn new(n: u32, m: u32) -> Self {
        ZnStarZmStar { n, m }
    }

    fn find_generators(&self) -> Vec<(u32, u32)> {
        let mut generators = Vec::new();
        
        let multiplicative_group_n = self.get_multiplicative_group(self.n);
        let multiplicative_group_m = self.get_multiplicative_group(self.m);

        for &i in &multiplicative_group_n {
            for &j in &multiplicative_group_m {
                if self.is_generator(i, j) {
                    generators.push((i, j));
                }
            }
        }

        generators
    }

    fn get_multiplicative_group(&self, modulus: u32) -> HashSet<u32> {
        let mut set = HashSet::new();
        for i in 1..modulus {
            if gcd(i, modulus) == 1 {
                set.insert(i);
            }
        }
        set
    }

    fn is_generator(&self, a: u32, b: u32) -> bool {
        let mut visited = HashSet::new();
        let phi_n = phi(self.n);
        let phi_m = phi(self.m);
        let mut count = 0;

        let mut x = 1;
        let mut y = 1;

        loop {
            if visited.contains(&(x, y)) {
                break;
            }

            visited.insert((x, y));
            count += 1;

            x = (x * a) % self.n;
            y = (y * b) % self.m;
        }

        count == (phi_n * phi_m)
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn phi(mut n: u32) -> u32 {
    let mut result = n;
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            result -= result / i;
        }
        i += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

fn main() {
    let group = ZnStarZmStar::new(2, 86);
    let generators = group.find_generators();

    println!("Generators of Z2^* x Z86^*: {:?}", generators);
}


