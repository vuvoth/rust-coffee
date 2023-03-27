use std::{fmt::Debug, ops::Deref};

#[derive(Debug)]
struct MyPointer {
    value: i32,
}

impl Deref for MyPointer {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Drop for MyPointer {
    fn drop(&mut self) {
        println!("Drop pointer with value {}", self.value);
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    #[test]
    fn deref_pointer_test() {
        let x = MyPointer { value: 124 };
        assert_eq!(*x, 124);
        let y = MyPointer { value: 10 };
        assert_eq!(*y, 10);
    }

    #[test]
    fn rc_pointer_test() {
        let x = Rc::new(10);
        {
            let _y = x.clone();
            println!("{}", Rc::strong_count(&x));
        }
        println!("{}", Rc::strong_count(&x));
    }
}
