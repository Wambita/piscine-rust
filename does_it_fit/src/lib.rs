mod areas_volumes;
use areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let container_area = x * y;
    let shape_area = match kind {
        GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b),
        GeometricalShapes::Circle => areas_volumes::circle_area(a),
    };
    shape_area * times as f64 <= container_area as f64
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let container_volume = x * y * z;
    let shape_volume = match kind {
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
        GeometricalVolumes::TriangularPyramid => {
            let base_area = a as f64; // assuming `a` is base_area
            areas_volumes::triangular_pyramid_volume(base_area, b)
        }
        GeometricalVolumes::Parallelepiped => {
            areas_volumes::parallelepiped_volume(a, b, c) as f64
        }
    };
    shape_volume * times as f64 <= container_volume as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::areas_volumes::{GeometricalShapes::*, GeometricalVolumes::*};

    #[test]
    fn test_area_fit() {
        assert!(!area_fit((2, 4), Rectangle, 100, (2, 1)));
        assert!(area_fit((5, 5), Triangle, 3, (5, 3)));
    }

    #[test]
    fn test_volume_fit() {
        assert!(!volume_fit((5, 5, 5), Sphere, 3, (2, 0, 0)));
        assert!(!volume_fit((5, 7, 5), Parallelepiped, 1, (6, 7, 4)));
    }
}
