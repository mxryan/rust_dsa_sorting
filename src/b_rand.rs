use std::sync::Mutex;

// https://blog.logrocket.com/rust-lazy-static-pattern/

lazy_static::lazy_static! {
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(34052));
}

pub fn rand(max: usize) -> usize {
    RG.lock().unwrap().next_v(max)
}

pub struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandGen {
    pub fn new(curr: usize) -> Self {
        RandGen {
            curr,
            mul: 56394237,
            inc: 346423491,
            modulo: 23254544563,
        }
    }
    pub fn next_v(&mut self, max: usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc) & self.modulo;
        self.curr % max
    }
}

#[cfg(test)]
mod tests {
    use crate::b_rand::RandGen;

    #[test]
    fn test_randy() {
        let mut r = RandGen::new(12);
        let mut itza_me_a_randum_number = 0;
        let mut total = 0;
        for _ in 0..100_000_000 {
            total += r.next_v(100);
        }
        println!("total: {}, average: {}", total, total as f64 / 100_000_000 as f64);
        itza_me_a_randum_number = r.next_v(100);
        assert_eq!(itza_me_a_randum_number, 3);
    }
}