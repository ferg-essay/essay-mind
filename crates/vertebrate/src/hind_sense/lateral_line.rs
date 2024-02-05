use crate::util::{Angle, DirVector, Line, Point};

fn dist_point_line(
    p: impl Into<Point>, 
    v: impl Into<Point>, 
    w: impl Into<Point>
) -> f32 {
    let p = p.into();
    let v = v.into();
    let w = w.into();

    Line(v, w).dist_point(p)
}

struct SenseArc {
    point: Point,
    dir1: Point,
    dir2: Point,
}

impl SenseArc {
    fn new(point: impl Into<Point>, dir: impl Into<Angle>) -> Self {
        let dir1 = Point::from(dir.into().sin_cos());
        let dir2 = Point(dir1.1, - dir1.0);

        Self {
            point: point.into(),
            dir1,
            dir2,
        }
    }

    pub fn sense_square(&self, square_ll: impl Into<Point>) -> DirVector {
        let ll = square_ll.into();

        let vector = self.dir_to(ll, (ll.0, ll.1 + 1.));
        let vector = best_vector(
            vector, 
            self.dir_to((ll.0, ll.1 + 1.), (ll.0 + 1., ll.1 + 1.))
        );
        let vector = best_vector(
            vector, 
            self.dir_to((ll.0 + 1., ll.1 + 1.), (ll.0 + 1., ll.1))
        );
        let vector = best_vector(
            vector, 
            self.dir_to((ll.0 + 1., ll.1), (ll.0, ll.1))
        );

        vector
    }

    fn dir_to(&self, v: impl Into<Point>, w: impl Into<Point>) -> DirVector {
        let vector = Line(v.into(), w.into());
        let proj = vector.projection(self.point);

        DirVector::new(self.point.angle_to(proj), self.point.dist(proj))
    }
}

fn best_vector(a: DirVector, b: DirVector) -> DirVector {
    if a.value() < b.value() {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod test {
    use crate::{hind_sense::lateral_line::SenseArc, util::DirVector};

    #[test]
    fn test_square() {
        let body = SenseArc::new((0., 0.), 0.);

        let dir = body.sense_square((1., 0.));
        assert_deq(dir, 0.25, 1.);
        let dir = body.sense_square((0., 1.));
        assert_deq(dir, 0., 1.);
        let dir = body.sense_square((-2., 0.));
        assert_deq(dir, 0.75, 1.);
        let dir = body.sense_square((0., -2.));
        assert_deq(dir, 0.5, 1.);

        let dir = body.sense_square((-1., 0.));
        assert_deq(dir, 0.0, 0.);
        let dir = body.sense_square((1., 1.));
        assert_deq(dir, 0.125, 1.4142135);
        let dir = body.sense_square((-2., -2.));
        assert_deq(dir, 0.625, 1.4142135);
    }

    fn assert_deq(a: DirVector, angle: f32, value: f32) {
        assert!(
            (a.value() - value).abs() < 1e-6 
            && (a.dir().to_unit() - angle).abs() < 1.0e-6,
            "({:?}, {:?}) != ({:?}, {:?})",
            a.dir().to_unit(), 
            a.value(),
            angle,
            value
        );
    }
}