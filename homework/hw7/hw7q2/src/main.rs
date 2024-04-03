fn main() {
    let sides_list = vec![6, 12, 24, 128, 256, 512, 1024, 2048, 65536];
    let lengths = vec![5.0, 10.0, 15.0];

    //loop over side number, then length for each side number
    for sides in &sides_list {
        for length in &lengths {
            let polygon = Polygon::new(*sides, *length);
            let polygon_area = polygon.area();
            let circle_area = 3.14 * polygon.apothem().powf(2.0);
            println!(
                "Polygon with {} sides and radius {} has area: {:.6}. Circle area: {:.6}",
                sides, length, polygon_area, circle_area
            );
            println!("Polygon is {:.4}% larger", 100.0 - (circle_area / polygon_area) * 100.0);
        }
    }
}

#[derive(Debug)]
struct Polygon {
    sides: u32,
    length: f32,
}

impl Polygon {
    fn new(sides: u32, length: f32) -> Polygon {
        Polygon{
            sides: sides,
            length: length,
        }
    }
}

trait RegularPolygon {
    fn perimeter(&self) -> f32;
    fn apothem(&self) -> f32;
    fn area(&self) -> f32;
    fn radius(&self) -> f32;
}

impl RegularPolygon for Polygon {
    fn perimeter(&self) -> f32 {
        self.length * self.sides as f32
    }
    fn apothem(&self) -> f32 {
        self.length / (2.0 * (180.0 / (self.sides as f32)).to_radians().tan())
    }
    fn area(&self) -> f32 {
        (self.apothem() * self.perimeter()) / 2.0
    }
    fn radius(&self) -> f32 {
        self.length / (2.0 * (180.0 / (self.sides as f32)).to_radians().sin())
    }
}