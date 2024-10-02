use std::f32::consts::PI;

pub struct HexOdorWorld {
    vec: Vec<HexOdor>,

    width: usize,
    height: usize,

    scale: f32,

    update_count: usize,
}

impl HexOdorWorld {
    pub fn new(width: usize, height: usize, scale: f32) -> HexOdorWorld {
        let hex_width = (width as f32 / scale + 1.) as usize;

        let hex_height = (height as f32 / scale * (PI / 6.).cos() + 1.) as usize;

        let mut vec = Vec::new();

        for _ in 0..hex_height {
            for _ in 0..hex_width {
                vec.push(HexOdor::default());
            }
        }

        Self {
            vec,
            width: hex_width,
            height: hex_height,
            scale: scale,
            update_count: 0,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn update_count(&self) -> usize {
        self.update_count
    }
}

pub struct HexOdor {
    kind: OdorKind,
}

impl Default for HexOdor {
    fn default() -> Self {
        Self { 
            kind: OdorKind::None,
        }
    }
}

pub enum OdorKind {
    None,
    A,
    B,
    C,
    D
}