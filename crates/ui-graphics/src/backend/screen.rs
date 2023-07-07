use essay_plot_base::CanvasEvent;

use super::canvas::{CanvasState, RendererApi};

pub trait ScreenApi {
    fn event(&mut self, canvas: &CanvasState, event: &CanvasEvent);
    
    fn tick(&mut self, canvas: &dyn RendererApi);

    fn draw(&mut self, canvas: &mut dyn RendererApi);
}