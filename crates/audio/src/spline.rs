pub struct BezierSpline {
    x: [f32; 4],
    y: [f32; 4],
}

impl BezierSpline {
    pub fn new(points: &[(f32, f32); 4]) -> Self {
        Self {
            x: [points[0].0, points[1].0, points[2].0, points[3].0],
            y: [points[0].1, points[1].1, points[2].1, points[3].1],
        }
    }

    pub fn eval(&self, t: f32) -> (f32, f32) {
        let x = &self.x;
        let y = &self.y;
        let mt = 1. - t;
        let t_2 = t * t;
        let mt_2 = mt * mt;

        let p_0 = mt_2 * mt;
        let p_1 = 3. * mt_2 * t;
        let p_2 = 3. * mt * t_2;
        let p_3 = t_2 * t;

        // print!("x0={} p0={} p1={} p2={} p3={}", x[0], p_0, p_1, p_2);

        let x_t = p_0 * x[0] + p_1 * x[1] + p_2 * x[2] + p_3 * x[3];
        let y_t = p_0 * y[0] + p_1 * y[1] + p_2 * y[2] + p_3 * y[3];

        (x_t, y_t)
    }

    pub fn eval_as_fn(&self, data: &mut [f32]) {
        let n = 2 * data.len();
        let len_f = data.len() as f32;
        let dt = 1. / n as f32;

        (_, data[0]) = self.eval(0.);
        let mut last_x: usize = 0;

        let mut t = 0.;

        for _ in 1..n {
            t += dt;

            let (x_t, y_t) = self.eval(t);
            let x_i = (x_t * len_f) as usize;

            while last_x + 1 < x_i {
                let x_m = 1. / (x_i - last_x) as f32;
                let y_m = (1. - x_m) * data[last_x] + x_m * y_t;

                last_x += 1;
                data[last_x] = y_m;
            }

            if last_x < x_i {
                last_x += 1;
                data[last_x] = y_t;
            }
        }
    }
}
