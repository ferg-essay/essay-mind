use std::time::{Duration, Instant};

use essay_plot_base::{Point};
use winit::{
    event::{ElementState, Event, MouseButton, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    dpi::PhysicalPosition, platform::run_return::EventLoopExtRunReturn,
};
use essay_ecs::app::App;

use crate::{backend::{
}, ui_canvas::UiCanvas};

pub fn main_loop(mut app: App) {
    // env_logger::init();

    let timer_length = Duration::from_millis(50);

    let event_loop = app.remove_resource_non_send::<EventLoop<()>>().unwrap();

    //app.insert_resource(WinitEvents::default());

    event_loop.run(move |event, _, control_flow| {
        app.resource_mut::<WinitEvents>().clear();

        //*control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => {
                control_flow.set_wait_until(Instant::now() + timer_length);
            }
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                control_flow.set_wait_until(Instant::now() + timer_length);

                app.update();
            }
            Event::MainEventsCleared => {
                // println!("Cleared");
            },
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                app.resource_mut::<UiCanvas>().window_bounds(size.width, size.height);
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                app.resource_mut::<WinitEvents>().mouse_button(state, button);
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                app.resource_mut::<WinitEvents>().cursor_event(position);
            }
            Event::RedrawRequested(_) => {
                //app.resource_mut::<UiCanvas>().set_stale();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

pub struct WinitEvents {
}

impl Default for WinitEvents {
    fn default() -> Self {
        Self {  

        }
    }
}

impl WinitEvents {
    pub fn clear(&mut self) {
        
    }

    fn mouse_button(&mut self, state: ElementState, button: MouseButton) {
        match button {
            MouseButton::Left => {
                // mouse.left = state;

                if state == ElementState::Pressed {
                //    let now = Instant::now();
                /*
                figure.event(
                    &mut draw_renderer,
                    &CanvasEvent::MouseLeftPress(cursor.position),
                );

                if now.duration_since(mouse.left_press_time).as_millis() < dbl_click {
                    figure.event(
                        &mut draw_renderer,
                        &CanvasEvent::ResetView(cursor.position),
                    )
                }
                */

                //mouse.left_press_start = cursor.position;
                //mouse.left_press_last = cursor.position;
                //mouse.left_press_time = now;
                //window.set_cursor_icon(CursorIcon::Grab);
                } else {
                    //window.set_cursor_icon(CursorIcon::Default);
                }
            }
            MouseButton::Right => {
                //mouse.right = state;

                match state {
                    ElementState::Pressed => {
                    /*
                    figure.event(
                        &mut render_state,
                        &CanvasEvent::MouseRightPress(cursor.position),
                    );

                    mouse.right_press_start = cursor.position;
                    mouse.right_press_time = Instant::now();
                    window.set_cursor_icon(CursorIcon::Crosshair);
                    */
                    }
                    ElementState::Released => {
                    /*
                    figure.event(
                        &mut draw_renderer,
                        &CanvasEvent::MouseRightRelease(cursor.position),
                    );
                    */

                    //if zoom_min <= mouse.right_press_start.dist(&cursor.position) {
                    /*
                    figure.event(
                        &mut draw_renderer,
                        &CanvasEvent::ZoomBounds(
                            mouse.right_press_start,
                            cursor.position
                        )
                    );
                    */
                    //}
                    //window.set_cursor_icon(CursorIcon::Default);
                    }
                }
            }
            _ => {}
        }
    }

    fn cursor_event(&mut self, position: PhysicalPosition<f64>) {
       //cursor.position = Point(
        //    position.x as f32,
        //    render_state.height() as f32 - position.y as f32,
        //);

        //if mouse.left == ElementState::Pressed
        //    && pan_min <= mouse.left_press_start.dist(&cursor.position)
        //{
        /*
        figure.event(
            &mut draw_renderer,
            &CanvasEvent::Pan(
                mouse.left_press_start,
                mouse.left_press_last,
                cursor.position
            ),
        );
        */

        //    mouse.left_press_last = cursor.position;
        //}

        //if mouse.right == ElementState::Pressed
        //   && pan_min <= mouse.left_press_start.dist(&cursor.position)
        // {
        /*
            figure.event(
                &mut draw_renderer,
                &CanvasEvent::MouseRightDrag(mouse.left_press_start, cursor.position),
        );
        */
        //}
    }
}


//use super::{render::{FigureRenderer, DrawRenderer}};

struct MouseState {
    left: ElementState,
    left_press_start: Point,
    left_press_last: Point,
    left_press_time: Instant,

    right: ElementState,
    right_press_start: Point,
    right_press_time: Instant,
}

impl MouseState {
    fn new() -> Self {
        Self {
            left: ElementState::Released,
            left_press_start: Point(0., 0.),
            left_press_last: Point(0., 0.),
            left_press_time: Instant::now(),

            right: ElementState::Released,
            right_press_start: Point(0., 0.),
            right_press_time: Instant::now(),
        }
    }
}

struct CursorState {
    position: Point,
}

impl CursorState {
    fn new() -> Self {
        Self {
            position: Point(0., 0.),
        }
    }
}

pub trait ViewRenderer {
    fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
        encoder: &wgpu::CommandEncoder,
    );
}
