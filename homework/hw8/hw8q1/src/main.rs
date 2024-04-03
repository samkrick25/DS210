use std::ops::Neg;
fn main() {
    let pointi32 = Point::create(1, 3);
    let pointf64 = Point::create(2.0, 4.98);
    println!("i32 Point: {:?}, rotated clockwise: {:?}", 
    pointi32, pointi32.clockwise());
    println!("f64 Point: {:?}, rotated counterclockwise: {:?}",
    pointf64, pointf64.counterclockwise())
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T:Copy + Neg<Output = T>> Point<T> {
    fn create(x: T, y: T) -> Point<T> {
        Point {x: x, y: y}
    }
    fn clockwise(&self) -> Point<T> {
        Point {x: self.y, y: -self.x}
    }
    fn counterclockwise(&self) -> Point<T> {
        Point {x: -self.y, y: self.x}
    }
}

#[test]
fn test_pos_clockwise() {
    let point = Point::create(2, 4);
    let rotated = &point.clockwise();
    assert_eq!(-point.x, rotated.y, "Value of rotated.y: {}", rotated.y);
    assert_eq!(point.y, rotated.x, "Value of rotated.x: {}", rotated.x);
}

#[test]
fn test_neg_clockwise() {
    let point = Point::create(-2, -4);
    let rotated = &point.clockwise();
    assert_eq!(-point.x, rotated.y, "Value of rotated.y: {}", rotated.y);
    assert_eq!(point.y, rotated.x, "Value of rotated.x: {}", rotated.x);
}

#[test]
fn test_pos_counterclockwise() {
    let point = Point::create(3, 5);
    let rotated = &point.counterclockwise();
    assert_eq!(point.x, rotated.y, "Value of rotated.y: {}", rotated.y);
    assert_eq!(-point.y, rotated.x, "Value of rotated.x: {}", rotated.x);
}

#[test]
fn test_neg_counterclockwise() {
    let point = Point::create(-3, -5);
    let rotated = &point.counterclockwise();
    assert_eq!(point.x, rotated.y, "Value of rotated.y: {}", rotated.y);
    assert_eq!(-point.y, rotated.x, "Value of rotated.x: {}", rotated.x);
}