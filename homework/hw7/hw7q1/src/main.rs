fn main() {
    let mut circle: Shape = create(vec![3.0]);
    let mut triangle: Shape = create(vec![2.0, 5.0, 6.5]);
    let mut rectangle: Shape = create(vec![4.0, 8.0]);

    //verify each instance of Shape
    circle.verify();
    println!();
    triangle.verify();
    println!();
    rectangle.verify();
    println!();

    // print each enum
    println!("Circle: {:?}, Triangle: {:?}, Rectangle: {:?}", &circle, &triangle, &rectangle);

    // print area
    println!("Circle Area: {:?}, Triangle area: {:?}, Rectangle area: {:?}", 
    &circle.area(), &triangle.area(), &rectangle.area());

    // print perimeter
    println!("Circle Circumference: {:?}, Triangle Perimeter: {:?}, Rectangle Perimeter: {:?}", 
    &circle.perimeter(), &triangle.perimeter(), &rectangle.perimeter());

    // double perimeter, then print again
    circle.double_perimeter();
    triangle.double_perimeter();
    rectangle.double_perimeter();
    println!("2x Circle Circumference: {:?}, 2x Triangle Perimeter: {:?}, 2x Rectangle Perimeter: {:?}", 
    &circle.perimeter(), &triangle.perimeter(), &rectangle.perimeter());
}

#[derive(Debug)]
enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
    Triangle(f32, f32, f32),
}

fn create(args: Vec<f32>) -> Shape {
    // the length of the input vector will tell which shape variant should be created
    match args.len() {
        1 => Shape::Circle(args[0]),
        2 => Shape::Rectangle(args[0], args[1]),
        3 => Shape::Triangle(args[0], args[1], args[2]),
        _ => panic!("Invalid number of arguments for creating a shape"),
    }
}

impl Shape {
    fn verify(&self) {
            //verify that no side lengths or radii are negative, and that triangle inequality holds
        match &self {
            Shape::Circle(radius) => {
                if *radius < 0.0 {panic!("Cannot have a negative radius!")}
                else {print!("All good!")}
            },
            Shape::Rectangle(length, width) => { 
                if *length < 0.0 || *width < 0.0 {panic!("Cannot have a negative length or width!")}
                else {print!("All good!")}
            },
            Shape::Triangle(s1, s2, s3) => {
                if *s1 < 0.0 || *s2 < 0.0 || *s3 < 0.0 {panic!("Cannot have a negative side length!")}
                if (*s1 + *s2) < *s3 {panic!("Not a valid triangle!")}
                else {println!("All good!")}
            }
        }
    }

    fn area(&self) -> f32 {
        // compute the area of each shape
        match &self {
            Shape::Circle(radius) => radius * 3.14,
            Shape::Rectangle(length, width) => length * width,
            Shape::Triangle(s1, s2, s3) => {
                let s: f32 = &self.perimeter() / 2.0 as f32;
                // use Heron's formula
                (s * (s - s1) * (s - s2) * (s - s3)).sqrt()
            },
        }
    }

    fn perimeter(&self) -> f32 {
        // compute shape perimeter
        match &self {
            Shape::Circle(radius) => 2.0 * 3.14 * radius,
            Shape::Rectangle(length, width) => (length * 2.0) + (width * 2.0),
            Shape::Triangle(s1, s2, s3) => s1 + s2 + s3,
        }
    }

    fn double_perimeter(&mut self) {
        //double perimeter of the object, this has no return value and modifies self in place
        match self {
            Shape::Circle(ref mut radius) => *radius *= 2.0,
            Shape::Rectangle(ref mut length, ref mut width) => {
                *length *= 2.0; *width *= 2.0
            },
            Shape::Triangle(ref mut s1, ref mut s2, ref mut s3) => {
                *s1 *= 2.0; *s2 *= 2.0; *s3 *= 2.0
            },
        }
    }
}
