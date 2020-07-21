struct Hex {
    q: i32,
    r: i32,
    s: i32,
}

impl Hex {
    fn new(q: i32, r: i32, s: i32) -> Hex {
        assert!(q + r + s, 0, "Initiation of Hex incorrect, q + r + s must equal 0");
        Hex(q, r, s)
    }
}
