use essay_graphics::{api::form::ShapeId, layout::MainLoop};
use essay_plot::api::{renderer::{self, Canvas, Drawable, Renderer}, Bounds, Color, Mesh2d};
use ui_graphics::{HexSliceGenerator, TexId, TextureBuilder, TextureGenerator};

fn main() { 
    // let mut layout = Layout::new();

    let colors = [
        Color::from("red"),
        Color::from("blue"),
        Color::from("orange"),
        Color::from("teal"),
    ];

    let s = 64;
    let mut tex_gen = TextureBuilder::new(s, s);

    for color in colors.iter() {
        let tile = tex_gen.create_tile();

        tex_gen.fill(tile, *color);

        tex_gen.tri_p(tile, "white", |u, v| {
            (u * 32.) as u32 % 2 == (v * 32.) as u32 % 2
        });

        let w = (10. * s as f32).recip();
        //tex_gen.tri(tile, "black", [0., 0.5], [1., 0.5], [1., 0.5 + w]);
        //tex_gen.tri(tile, "black", [0., 0.5], [0., 0.5 + w], [1., 0.5 + w]);

        tex_gen.quad(tile, "black", [0., 0.5], [1., 0.5], [1., 0.5 + w], [0., 0.5 + w]);

        tex_gen.tri(tile, "black", [0., 0.0], [1., 0.0], [1., 0.0 + w]);
        tex_gen.tri(tile, "black", [0., 0.0], [0., 0.0 + w], [1., 0.0 + w]);
    }

    let tex = tex_gen.gen();

    let mut form = Mesh2d::new();

    let gen = HexSliceGenerator::new(0.1, 0.1);

    gen.hex(&mut form, [0.251, 0.25], tex.tile(TexId(0)));
    gen.hex(&mut form, [0.40, 0.25 + 0.0866], tex.tile(TexId(1)));
    gen.hex(&mut form, [0.55, 0.25], tex.tile(TexId(2)));
    gen.hex(&mut form, [0.40, 0.25 - 0.0866], tex.tile(TexId(3)));

    /*
    layout.view(((0.5, 0.5), [0.5, 0.5]),
        ShapeView::new(form, tex)
    );
    */

    MainLoop::new().show(ShapeView::new(form, tex));
}

struct ShapeView {
    form: Mesh2d,
    _form_id: Option<ShapeId>,
    texture: TextureGenerator,

    is_dirty: bool,
}

impl ShapeView {
    fn new(form: Mesh2d, texture: TextureGenerator) -> Self {
        Self {
            form,
            _form_id: None,
            texture,
            is_dirty: true,
        }
    }

    fn fill_model(&mut self, renderer: &mut dyn Renderer) {
        self.texture.bind(renderer);
        //self.form.texture(self.texture.texture_id());

        //self.form_id = Some(renderer.create_shape(&self.form));
    }
}

impl Drawable for ShapeView {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        if self.is_dirty {
            self.is_dirty = false;
            self.fill_model(renderer);
        }

        //if let Some(id) = self.form_id {
        let canvas = renderer.pos().clone();
        let bounds = Bounds::<Canvas>::from(([0., 0.], [1., 1.]));
        let camera = bounds.affine_to(&canvas);
            
        renderer.draw_mesh2d(&self.form, self.texture.texture_id(), &[camera.into()])?;

        Ok(())
    }
}
