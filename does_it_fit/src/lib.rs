pub mod areas_volumes; // Import the module containing area and volume calculation functions
pub use areas_volumes::*; // Re-export everything from the module for use in this file

// Determines if a given number of 2D shapes can fit within a 2D container of size x * y
pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let max_size = x * y; // Calculate total available area
    let size = match kind {
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Circle => circle_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b),
    };
    times as f64 * size <= max_size as f64
}

// Determines if a given number of 3D volumes can fit within a 3D container of size x * y * z
pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let max_size = x * y * z; // Calculate total available volume
    let size = match kind {
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a),
        GeometricalVolumes::Cone => cone_volume(a, b),
        GeometricalVolumes::TriangularPyramid => triangular_pyramid_volume(triangle_area(a, b), c),
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
    };
    times as f64 * size <= max_size as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_volumes_shapes() {
        assert_eq!(true, area_fit((2, 5), GeometricalShapes::Circle, 0, (2, 1)));
        assert_eq!(true, area_fit((2, 2), GeometricalShapes::Rectangle, 0, (6, 10)));
        assert_eq!(true, volume_fit((2, 5, 3), GeometricalVolumes::Cone, 0, (1, 1, 1)));
        assert_eq!(true, volume_fit((3, 5, 7), GeometricalVolumes::Parallelepiped, 0, (2, 6, 3)));
    }

    #[test]
    fn equal_size() {
        assert_eq!(true, area_fit((2, 5), GeometricalShapes::Square, 1, (2, 5)));
        assert_eq!(true, volume_fit((3, 1, 4), GeometricalVolumes::Cube, 1, (1, 3, 4)));
    }

    #[test]
    fn all_shapes() {
        assert_eq!(false, area_fit((3, 5), GeometricalShapes::Circle, 2, (2, 0)));
        assert_eq!(true, area_fit((8, 6), GeometricalShapes::Triangle, 5, (5, 2)));
        assert_eq!(true, area_fit((7, 3), GeometricalShapes::Rectangle, 2, (2, 4)));
        assert_eq!(true, area_fit((5, 5), GeometricalShapes::Square, 1, (2, 4)));
    }

    #[test]
    fn all_volumes() {
        assert_eq!(true, volume_fit((5, 6, 3), GeometricalVolumes::Cube, 2, (3, 3, 4)));
        assert_eq!(false, volume_fit((7, 4, 4), GeometricalVolumes::Cone, 1, (8, 2, 0)));
        assert_eq!(true, volume_fit((2, 5, 3), GeometricalVolumes::Sphere, 1, (1, 1, 1)));
        assert_eq!(false, volume_fit((2, 5, 3), GeometricalVolumes::Parallelepiped, 31, (1, 1, 1)));
        assert_eq!(true, volume_fit((7, 5, 3), GeometricalVolumes::TriangularPyramid, 3, (3, 2, 1)));
    }
}