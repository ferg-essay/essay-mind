use std::f32::consts::TAU;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point(pub f32, pub f32);

impl Point {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn is_below(&self, p0: &Point, p1: &Point) -> bool {
        let Point(x, y) = self;
        let Point(x0, y0) = p0;
        let Point(x1, y1) = p1;

        if x0 == x1 {
            false
        } else if x0 <= x && x < x1 || x1 < x && x <= x0 {
            let y_line = (y0 * (x1 - x) + y1 * (x - x0)) / (x1 - x0);

            *y < y_line
        } else {
            false
        }
    }

    #[inline]
    pub fn dist(&self, p: &Point) -> f32 {
        let dx = self.0 - p.0;
        let dy = self.1 - p.1;

        dx.hypot(dy)
    }
}

impl From<[f32; 2]> for Point {
    #[inline]
    fn from(value: [f32; 2]) -> Self {
        Point(value[0], value[1])
    }
}

impl From<(f32, f32)> for Point {
    #[inline]
    fn from(value: (f32, f32)) -> Self {
        Point(value.0, value.1)
    }
}

impl From<&[f32; 2]> for Point {
    #[inline]
    fn from(value: &[f32; 2]) -> Self {
        Point(value[0], value[1])
    }
}

// angle in [0., 1.]
#[derive(Clone, Copy, Debug)]
pub enum Angle {
    Rad(f32),
    Deg(f32),
    Unit(f32),
}

impl Angle {
    #[inline]
    pub fn to_radians(&self) -> f32 {
        match self {
            Angle::Rad(rad) => (*rad + TAU) % TAU,
            Angle::Deg(deg) => ((90. - deg).to_radians() + TAU) % TAU,
            Angle::Unit(unit) => ((1.25 - unit) * TAU) % TAU,
        }
    }

    #[inline]
    pub fn to_degrees(&self) -> f32 {
        match self {
            Angle::Rad(rad) => (rad.to_degrees() + 360.) % 360.,
            Angle::Deg(deg) => (*deg + 360.) % 360.,
            Angle::Unit(unit) => (unit * 360. + 360.) % 360.,
        }
    }

    #[inline]
    pub fn to_unit(&self) -> f32 {
        match self {
            Angle::Rad(rad) => (rad.to_degrees() / 360. + 1.) % 1.,
            Angle::Deg(deg) => (deg / 360. + 1.) % 1.,
            Angle::Unit(unit) => (*unit + 1.) % 1.,
        }
    }

    #[inline]
    pub fn to_unit_zero(&self) -> f32 {
        match self {
            Angle::Rad(rad) => (1.5 - rad / TAU) % 1. - 0.5,
            Angle::Deg(deg) => (deg / 360. + 1.5) % 1. - 0.5,
            Angle::Unit(unit) => (unit + 1.5) % 1. - 0.5,
        }
    }

    #[inline]
    pub fn cos(&self) -> f32 {
        self.to_radians().cos()
    }

    #[inline]
    pub fn sin(&self) -> f32 {
        self.to_radians().sin()
    }

    #[inline]
    pub fn sin_cos(&self) -> (f32, f32) {
        self.to_radians().sin_cos()
    }
}

impl From<f32> for Angle {
    fn from(value: f32) -> Self {
        Angle::Rad(value)
    }
}

#[cfg(test)]
mod test {
    use std::f32::consts::TAU;

    use super::Angle;

    #[test]
    fn unit_angle() {
        assert_feq(Angle::Unit(0.).to_radians(), 0.25 * TAU);
        assert_feq(Angle::Unit(0.25).to_radians(), 1e-6);
        assert_feq(Angle::Unit(0.5).to_radians(), 0.75 * TAU);
        assert_feq(Angle::Unit(0.75).to_radians(), 0.5 * TAU);
    }

    #[test]
    fn unit_sincos() {
        assert_feq(Angle::Unit(0.).sin(), 1.);
        assert_feq(Angle::Unit(0.25).sin(), 0.);
        assert_feq(Angle::Unit(0.5).sin(), -1.);
        assert_feq(Angle::Unit(0.75).sin(), 0.);

        assert_feq(Angle::Unit(0.).cos(), 0.);
        assert_feq(Angle::Unit(0.25).cos(), 1.);
        assert_feq(Angle::Unit(0.5).cos(), 0.);
        assert_feq(Angle::Unit(0.75).cos(), -1.);

        assert_feq2(Angle::Unit(0.).sin_cos(), (1., 0.));
        assert_feq2(Angle::Unit(0.25).sin_cos(), (0., 1.));
        assert_feq2(Angle::Unit(0.5).sin_cos(), (-1., 0.));
        assert_feq2(Angle::Unit(0.75).sin_cos(), (0., -1.));
    }

    #[test]
    fn deg_angle() {
        assert_feq(Angle::Deg(0.).to_radians(), 0.25 * TAU);
        assert_feq(Angle::Deg(90.).to_radians(), 1e-6);
        assert_feq(Angle::Deg(180.).to_radians(), 0.75 * TAU);
        assert_feq(Angle::Deg(270.).to_radians(), 0.5 * TAU);
    }

    fn assert_feq(left: f32, right: f32) {
        assert!((left - right).abs() < 1e-5,
                "assertion failed {} == {}",
                left, right);
    }

    fn assert_feq2(left: (f32, f32), right: (f32, f32)) {
        assert!((left.0 - right.0).abs() < 1e-5
            && (left.1 - right.1).abs() < 1e-5,
            "assertion failed {:?} == {:?}",
            left, right);
    }
}