pub mod areas_volumes {
    pub enum GeometricalShapes {
        Square,
        Circle,
        Rectangle,
        Triangle,
    }

    pub enum GeometricalVolumes {
        Cube,
        Sphere,
        Cone,
        TriangularPyramid,
        Parallelepiped,
    }

    pub(crate) fn square_area(side: usize) -> usize {
        side.pow(2)
    }

    pub(crate) fn triangle_area(base: usize, height: usize) -> f64 {
        (base as f64 * height as f64) / 2.0
    }

    pub(crate) fn circle_area(radius: usize) -> f64 {
        std::f64::consts::PI * (radius.pow(2) as f64)
    }

    pub(crate) fn rectangle_area(side_a: usize, side_b: usize) -> usize {
        side_a * side_b
    }

    pub(crate) fn cube_volume(side: usize) -> usize {
        side.pow(3)
    }

    pub(crate) fn sphere_volume(radius: usize) -> f64 {
        (4.0 / 3.0) * std::f64::consts::PI * (radius.pow(3) as f64)
    }

    pub(crate) fn triangular_pyramid_volume(base_area: f64, height: usize) -> f64 {
        (base_area * height as f64) / 3.0
    }

    pub(crate) fn parallelepiped_volume(side_a: usize, side_b: usize, side_c: usize) -> usize {
        side_a * side_b * side_c
    }

    pub(crate) fn cone_volume(base_radius: usize, height: usize) -> f64 {
        (1.0 / 3.0) * std::f64::consts::PI * base_radius.pow(2) as f64 * height as f64
    }
}

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let rectangle_area = x * y;
    let shape_area: f64 = match kind {
        areas_volumes::GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        areas_volumes::GeometricalShapes::Circle => areas_volumes::circle_area(a),
        areas_volumes::GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        areas_volumes::GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b),
    };
    shape_area * times as f64 <= rectangle_area as f64
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let box_volume = x * y * z;
    let shape_volume: f64 = match kind {
        areas_volumes::GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        areas_volumes::GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        areas_volumes::GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
        areas_volumes::GeometricalVolumes::TriangularPyramid => {
            areas_volumes::triangular_pyramid_volume(a as f64, b)
        }
        areas_volumes::GeometricalVolumes::Parallelepiped => {
            areas_volumes::parallelepiped_volume(a, b, c) as f64
        }
    };
    shape_volume * times as f64 <= box_volume as f64
}