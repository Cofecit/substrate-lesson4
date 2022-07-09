#![allow(dead_code, unused_imports)]

// Graphic trait, has a area function
pub trait Graphic {
    // method to get graphic area
    fn area(&self) -> f64;
}

// Circle with center point and radius
pub struct Circle((u32, u32), f64);

// Graphic implementation of Circle
impl Graphic for Circle {
    fn area(&self) -> f64 {
        // area = PI * radius * radius
        return std::f64::consts::PI * self.1 * self.1;
    }
}

// Triangle with three point
pub struct Triangle((u32, u32), (u32, u32), (u32, u32));

// Graphic implementation of Triangle
impl Graphic for Triangle {
    fn area(&self) -> f64 {
        // area = ( 1 / 2 ) * ( x1y2 + x2y3 + x3y1 - x1y3 - x2y1 - x3y2 )
        return 0.5
            * (self.0 .0 * self.1 .1 + self.1 .0 * self.2 .1 + self.2 .0 * self.0 .1
                - self.0 .0 * self.2 .1
                - self.1 .0 * self.0 .1
                - self.2 .0 * self.1 .1) as f64;
    }
}

// Rectangle in Polar coordinates with top left point and bottom right point
pub struct Rectangle((u32, u32), (u32, u32));

// Graphic implementation of Rectangle
impl Graphic for Rectangle {
    fn area(&self) -> f64 {
        // area = (x2 - x1) * (y2 - y1)
        return ((self.1 .0 - self.0 .0) * (self.1 .1 - self.0 .1)) as f64;
    }
}

// function to calculate graphic area
pub fn calculate_area<T>(graphic: T) -> f64
where
    T: Graphic,
{
    return graphic.area();
}

/////////////////////////// test case for graphic area ///////////////////////////
#[cfg(test)]
mod tests {
    use crate::graphic_area::*;

    #[test]
    fn test_graphic_area() {
        let circle = Circle((0, 0), 2_f64);
        let triagnle = Triangle((0, 0), (2, 0), (0, 2));
        let rectangle = Rectangle((0, 0), (4, 4));

        assert_eq!(calculate_area(circle), std::f64::consts::PI * 2_f64 * 2_f64);
        assert_eq!(calculate_area(triagnle), 2_f64);
        assert_eq!(calculate_area(rectangle), 16_f64);
    }
}
