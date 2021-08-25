


#[cfg(test)]
mod tests {

    pub struct Rect {
        width: u32,
        height: u32,
    }
    
    
    impl Rect {
    
        pub fn can_hold(&self, other: &Rect) -> bool {
            return (self.width > other.width) && (self.height > other.height)
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn rect_test() {
        let rect1 = Rect {
            width: 8,
            height: 7,
        };

        let rect2 = Rect {
            width: 5,
            height: 1,
        };

        assert!(rect1.can_hold(&rect2));
    }
}
