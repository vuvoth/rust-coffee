trait Area {
    fn compute_area(self) -> u64;
}

struct Rectangle {
    h: u64,
    l: u64,
}
struct Square {
    n: u64,
}

impl Area for Square {
    fn compute_area(self) -> u64 {
        return self.n * self.n;
    }
}

impl Area for Rectangle {
    fn compute_area(self) -> u64 {
        return self.h * self.l;
    }
}

#[cfg(test)]
mod tests {

    use super::{Area, Rectangle, Square};
    #[test]
    pub fn it_works() {
        let square = Square { n: 10 };

        assert_eq!(square.compute_area(), 100);

        let rectangle = Rectangle { h: 10, l: 20 };

        assert_eq!(rectangle.compute_area(), 200);
    }
}
