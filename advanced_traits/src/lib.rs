use std::ops::Add;

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.count;
        self.count += 1;
        Some(current)
    }
}

struct Millimetres(u32);
struct Metres(u32);

impl Add<Metres> for Millimetres {
    type Output = Millimetres;

    fn add(self, rhs: Metres) -> Self::Output {
        Millimetres(self.0 + (rhs.0 * 1000))
    }
}
