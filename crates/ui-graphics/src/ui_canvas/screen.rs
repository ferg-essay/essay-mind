use essay_plot::api::renderer::Event;

use super::canvas::{CanvasState, RendererApi};

pub trait ScreenApi {
    fn event(&mut self, canvas: &CanvasState, event: &Event);
    
    fn tick(&mut self);

    fn draw(&mut self, canvas: &mut dyn RendererApi);
}