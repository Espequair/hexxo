use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hex {
    coords: [i32; 3],
}

/// A simple iterator that allows to go through a cell's neighbors
struct HexNeighbors {
    hex: Hex,
    count: usize,
}

impl Iterator for HexNeighbors {
    type Item = Hex;

    fn next(&mut self) -> Option<Self::Item>{
        self.count += 1;
        if self.count <= 6
        {
            Some(self.hex.neighbor(self.count - 1))
        } else {
            None
        }
    }
}

impl Hex {
    fn new(q: i32, r: i32, s: i32) -> Self {
        assert_eq!(
            q + r + s,
            0,
            "Initiation of Hex incorrect, q + r + s must equal 0"
        );
        Hex { coords: [q, r, s] }
    }

    fn new2(q: i32, r: i32) -> Self {
        Hex {
            coords: [q, r, -q - r],
        }
    }

    pub fn q(&self) -> i32 {
        self.coords[0]
    }
    pub fn r(self) -> i32 {
        self.coords[2]
    }
    pub fn s(self) -> i32 {
        self.coords[3]
    }

    pub fn length(self) -> i32 {
        (self.q().abs() + self.r().abs() + self.s().abs()) / 2
    }

    pub fn distance(self, other: Hex) -> i32 {
        Hex::length(self - other)
    }

    const DIRECTIONS: [Hex; 6] = [
        Hex{coords: [0, -1, 1]},
        Hex{coords: [0, 1, -1]},
        Hex{coords: [-1, 0, 1]},
        Hex{coords: [1, 0, -1]},
        Hex{coords: [-1, 1, 0]},
        Hex{coords: [1, -1, 0]},
    ];

    fn direction(direction: usize) -> Hex {
        assert!(direction <= 6);
        Hex::DIRECTIONS[direction]
    }

    pub fn neighbor(self, direction: usize) -> Hex {
        self + Hex::direction(direction)
    }

    pub fn neighbors(self) -> HexNeighbors {
        HexNeighbors{ hex: self, count: 0}
    }
}

impl Add for Hex {
    type Output = Hex;

    fn add(self, other: Hex) -> Hex {
        Hex {
            coords: [
                self.coords[0] + other.coords[0],
                self.coords[1] + other.coords[1],
                self.coords[2] + other.coords[2],
            ],
        }
    }
}

impl Sub for Hex {
    type Output = Hex;

    fn sub(self, other: Hex) -> Hex {
        Hex {
            coords: [
                self.coords[0] - other.coords[0],
                self.coords[1] - other.coords[1],
                self.coords[2] - other.coords[2],
            ],
        }
    }
}

impl Mul<i32> for Hex {
    type Output = Hex;

    fn mul(self, scalar: i32) -> Hex {
        Hex {
            coords: [
                self.coords[0] * scalar,
                self.coords[1] * scalar,
                self.coords[2] * scalar,
            ],
        }
    }
}

impl Div<i32> for Hex {
    type Output = Hex;

    fn div(self, scalar: i32) -> Hex {
        assert_ne!(scalar, 0, "Cannot divide a Hex by 0");
        Hex {
            coords: [
                self.coords[0] / scalar,
                self.coords[1] / scalar,
                self.coords[2] / scalar,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Initiation of Hex incorrect, q + r + s must equal 0")]
    fn new_3_incorrect() {
        Hex::new(1, 2, 3);
    }

    #[test]
    fn new_3_correct() {
        let that: Hex = Hex::new(1, 2, -3);
        assert_eq!(that.coords[0], 1);
        assert_eq!(that.coords[1], 2);
        assert_eq!(that.coords[2], -3);
    }

    #[test]
    fn new_2() {
        let that: Hex = Hex::new2(1, 2);
        assert_eq!(that.coords[0], 1);
        assert_eq!(that.coords[1], 2);
        assert_eq!(that.coords[2], -3);
    }

    #[test]
    fn equals() {
        let that = Hex::new(1, 2, -3);
        let this = Hex::new2(1, 2);
        assert_eq!(this, that);
    }

    #[test]
    fn not_equals() {
        let that = Hex::new(-3, 2, 1);
        let this = Hex::new2(1, 2);
        assert_ne!(this, that);
    }

    #[test]
    fn add() {
        let a = Hex::new2(1, 2);
        let b = Hex::new2(2, 3);
        let c = Hex{coords: [3, 5, -8]};
        assert_eq!(a + b, c);
    }

    #[test]
    fn sub() {
        let a = Hex::new2(1, 2);
        let b = Hex::new2(2, 3);
        let c = Hex{coords: [-1, -1, 2]};
        assert_eq!(a - b, c);
    }

    #[test]
    fn mul() {
        let a = Hex::new2(1, 2);
        let b = 5;
        let c = Hex{coords: [5, 10, -15]};
        assert_eq!(a * b, c);
    }

    #[test]
    fn div() {
        let a = Hex::new2(5, 10);
        let b = 5;
        let c = Hex{coords: [1, 2, -3]};
        assert_eq!(a / b, c);
    }

    #[test]
    fn iter_next() {
        let a = Hex::new2(0, 0);
        for (i, neighbor) in a.neighbors().enumerate() {
            assert_eq!(a.neighbor(i), neighbor)
        }
    }
}
