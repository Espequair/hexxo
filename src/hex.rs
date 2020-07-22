#[derive(Debug,Clone, Copy, PartialEq, Eq)]
struct Hex {
    coords: [i32; 3],
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
        let that = Hex::new(1, 2, 3);
        let this = Hex::new2(1, 2);
        assert_ne!(this, that);
    }
}
