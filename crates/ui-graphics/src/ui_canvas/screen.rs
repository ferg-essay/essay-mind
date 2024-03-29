use essay_plot::api::CanvasEvent;

use super::canvas::{CanvasState, RendererApi};

pub trait ScreenApi {
    fn event(&mut self, canvas: &CanvasState, event: &CanvasEvent);
    
    fn tick(&mut self);

    fn draw(&mut self, canvas: &mut dyn RendererApi);
}