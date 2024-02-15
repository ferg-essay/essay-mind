use std::{f32::consts::TAU, ops::{Add, Mul, Sub}};

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
    pub fn dist(&self, p: impl Into<Point>) -> f32 {
        let p = p.into();

        let dx = self.0 - p.0;
        let dy = self.1 - p.1;

        dx.hypot(dy)
    }

    #[inline]
    pub fn dist_square(&self, p: impl Into<Point>) -> f32 {
        let p = p.into();

        let dx = self.0 - p.0;
        let dy = self.1 - p.1;

        dx * dx + dy * dy
    }

    #[inline]
    pub fn dot(&self, p: impl Into<Point>) -> f32 {
        let p = p.into();

        self.0 * p.0 + self.1 * p.1
    }

    #[inline]
    pub fn angle_to(&self, pos: Point) -> Angle {
        Angle::Rad((pos.x() - self.x()).atan2(pos.y() - self.y()))
    }

    #[inline]
    pub fn tri_det(&self, b: impl Into<Point>, c: impl Into<Point>) -> f32 {
        let b = b.into();
        let c = c.into();

        self.0 * (b.1 - c.1) + b.0 * (c.1 - self.1) + c.0 * (self.1 - b.1)
    }
}

impl From<&Point> for Point {
    #[inline]
    fn from(value: &Point) -> Self {
        value.clone()
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

impl Add<Point> for Point {
    type Output = Point;

    #[inline]
    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    #[inline]
    fn sub(self, rhs: Point) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<Point> for f32 {
    type Output = Point;

    #[inline]
    fn mul(self, rhs: Point) -> Self::Output {
        Point(self * rhs.0, self * rhs.1)
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
    pub fn to_turn(&self) -> f32 {
        (self.to_unit() + 0.5) % 1. - 0.5
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

    #[inline]
    pub fn unit(dir: f32) -> Angle {
        Self::Unit((dir + 1.) % 1.)
    }
}

impl From<f32> for Angle {
    fn from(value: f32) -> Self {
        Angle::Rad(value)
    }
}

impl Add<Angle> for Angle {
    type Output = Angle;

    fn add(self, rhs: Angle) -> Self::Output {
        Angle::unit(self.to_unit() + rhs.to_unit())
    }
}

impl Sub<Angle> for Angle {
    type Output = Angle;

    fn sub(self, rhs: Angle) -> Self::Output {
        Angle::unit(self.to_unit() - rhs.to_unit())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line(pub Point, pub Point);

impl Line {
    #[inline]
    pub fn len(&self) -> f32 {
        self.0.dist(self.1)
    }

    #[inline]
    pub fn len_square(&self) -> f32 {
        self.0.dist_square(self.1)
    }

    #[inline]
    pub fn vector(&self) -> Point {
        self.1 - self.0
    }

    #[inline]
    pub fn dist_point(&self, p: impl Into<Point>) -> f32 {
        let p = p.into();

        p.dist(self.projection(p))
    }

    #[inline]
    pub fn projection(&self, p: impl Into<Point>) -> Point {
        let p = p.into();
        let v = self.0;
        let w = self.1;
    
        let l2 = v.dist_square(w);

        if l2 == 0. {
            return v
        }

        let t = ((p - v).dot(w - v) / l2).clamp(0., 1.);

        v + t * (w - v)
    }

    #[inline]
    pub fn tri_det(&self, p: impl Into<Point>) -> f32 {
        self.0.tri_det(self.1, p.into())
    }
}

impl From<(Point, Point)> for Line {
    #[inline]
    fn from(value: (Point, Point)) -> Self {
        Line(value.0, value.1)
    }
}

#[cfg(test)]
mod test {
    use std::f32::consts::TAU;

    use super::{Angle, Point};

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

    #[test]
    fn point_tri_det() {
        assert_feq(Point(0., 0.).tri_det((0., 1.), (1., 0.)), -1.);

        assert_feq(Point(0., 0.).tri_det((0., 1.), (-1., 0.)), 1.);

        assert_feq(Point(0., 1.).tri_det((0., 0.), (1., 0.)), 1.);

        assert_feq(Point(0., 1.).tri_det((0., 0.), (-1., 0.)), -1.);

        assert_feq(Point(0., 0.).tri_det((0., 1.), (0., 10.)), 0.);
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