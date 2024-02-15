use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{body::Body, tectum::tectum::TectumMap, util::{Angle, DirVector, Line, Point}, world::World};

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
    dir: Angle,
    // dir2: Point,
}

impl SenseArc {
    fn new(point: impl Into<Point>, dir: impl Into<Angle>) -> Self {
        //let dir1 = Point::from(dir.into().sin_cos());
        //let dir2 = Point(dir1.1, - dir1.0);

        Self {
            point: point.into(),
            dir: dir.into(),
            // dir2,
        }
    }

    fn update(
        &self, 
        dx: f32, 
        dy: f32, 
        world: &World, 
        tectum: &mut TectumMap
    ) {
        let pos = self.point + Point(dx, dy);

        if world.is_collide(pos) {
            let Point(x, y) = pos;

            let vector = self.sense_square((x.floor(), y.floor()));
            let dir = vector.dir();// - self.dir;
            let value = (1. - vector.value()).clamp(0., 1.);

            tectum.neg(dir, value);
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

fn update_lateral_line(
    body: Res<Body>,
    world: Res<World>,
    mut tectum: ResMut<TectumMap>
) {
    // let Point(x, y) = body.pos();

    let sense = SenseArc::new(body.pos(), body.dir());
    // let sense = SenseArc::new(Point(0.1, 0.1), body.dir());

    sense.update(-1., -1., world.get(), tectum.get_mut());
    sense.update(0., -1., world.get(), tectum.get_mut());
    sense.update(1., -1., world.get(), tectum.get_mut());

    sense.update(-1., 0., world.get(), tectum.get_mut());
    // sense.update(0., 0, world.get(), tectum.get_mut());
    sense.update(1., 0., world.get(), tectum.get_mut());

    sense.update(-1., 1., world.get(), tectum.get_mut());
    sense.update(0., 1., world.get(), tectum.get_mut());
    sense.update(1., 1., world.get(), tectum.get_mut());
}

pub struct LateralLinePlugin;

impl Plugin for LateralLinePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_resource::<TectumMap>());
        //assert!(app.ini.resour)
        //app.init_resource::<MidMotor>();
        //app.event::<MidMotorEvent>();

        app.system(Tick, update_lateral_line);
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

    #[test]
    fn test_surround() {
        let body = SenseArc::new((0.5, 0.5), 0.);

        let dir = body.sense_square((-1., 0.));
        assert_deq(dir, 0.75, 0.5);
        let dir = body.sense_square((1., 0.));
        assert_deq(dir, 0.25, 0.5);

        let dir = body.sense_square((0., 1.));
        assert_deq(dir, 0., 0.5);
        let dir = body.sense_square((0., -1.));
        assert_deq(dir, 0.5, 0.5);

        let dir = body.sense_square((-1., 1.));
        assert_deq(dir, 0.875, 2.0f32.sqrt().recip());
        let dir = body.sense_square((1., 1.));
        assert_deq(dir, 0.125, 2.0f32.sqrt().recip());

        let dir = body.sense_square((-1., -1.));
        assert_deq(dir, 0.625, 2.0f32.sqrt().recip());
        let dir = body.sense_square((1., -1.));
        assert_deq(dir, 0.375, 2.0f32.sqrt().recip());
    }

    #[test]
    fn test_surround_10_20() {
        let (x, y) = (10., 20.);

        let body = SenseArc::new((x + 0.5, y + 0.5), 0.);

        let dir = body.sense_square((x - 1., y + 0.));
        assert_deq(dir, 0.75, 0.5);
        let dir = body.sense_square((x + 1., y + 0.));
        assert_deq(dir, 0.25, 0.5);

        let dir = body.sense_square((x + 0., y + 1.));
        assert_deq(dir, 0., 0.5);
        let dir = body.sense_square((x + 0., y - 1.));
        assert_deq(dir, 0.5, 0.5);

        let dir = body.sense_square((x - 1., y + 1.));
        assert_deq(dir, 0.875, 2.0f32.sqrt().recip());
        let dir = body.sense_square((x + 1., y + 1.));
        assert_deq(dir, 0.125, 2.0f32.sqrt().recip());

        let dir = body.sense_square((x - 1., y - 1.));
        assert_deq(dir, 0.625, 2.0f32.sqrt().recip());
        let dir = body.sense_square((x + 1., y - 1.));
        assert_deq(dir, 0.375, 2.0f32.sqrt().recip());
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