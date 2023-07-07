use std::time::{Instant, Duration};

use essay_plot_base::{Point, CanvasEvent};
use winit::{
    event::{Event, WindowEvent, ElementState, MouseButton, StartCause },    
    event_loop::{EventLoop, ControlFlow}, 
    window::{Window, CursorIcon},
};

use crate::backend::{screen::ScreenApi, canvas::{CanvasState}, wgpu_canvas::{WgpuCanvas, init_wgpu_args}};

pub fn main_loop(screen: impl ScreenApi + 'static) {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    env_logger::init();
    let wgpu_canvas = pollster::block_on(init_wgpu_args(&window));

    run_event_loop(event_loop, window, wgpu_canvas, Box::new(screen));
}

fn run_event_loop(
    event_loop: EventLoop<()>, 
    window: Window, 
    wgpu_canvas: WgpuCanvas,
    screen: Box<dyn ScreenApi>,
) {
    let mut figure = screen;

    let mut render_state = CanvasState::new(
        wgpu_canvas
    );

    let pan_min = 20.;
    let zoom_min = 20.;

    // TODO: is double clicking no longer recommended?
    let dbl_click = 500; // time in millis

    let mut cursor = CursorState::new();
    let mut mouse = MouseState::new();
    let mut last_time = Instant::now();
    let timer_length = Duration::from_millis(50);

    event_loop.run(move |event, _, control_flow| {
        let mut is_draw = false;
    
        //*control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => {
                control_flow.set_wait_until(Instant::now() + timer_length);
            },
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                control_flow.set_wait_until(Instant::now() + timer_length);
                // figure.tick(&mut render_state.renderer());
                render_state.tick(&mut figure);
            },
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                //wgpu_canvas.window_bounds(size.width, size.height);
                render_state.set_canvas_bounds(size.width, size.height);
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput {
                    state,
                    button,
                    ..
                },
                ..
            } => {
                match button {
                    MouseButton::Left => {
                        mouse.left = state;

                        if state == ElementState::Pressed {
                            let now = Instant::now();
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

                            mouse.left_press_start = cursor.position;
                            mouse.left_press_last = cursor.position;
                            mouse.left_press_time = now;
                            window.set_cursor_icon(CursorIcon::Grab);
                        } else {
                            window.set_cursor_icon(CursorIcon::Default);
                        }
                    },
                    MouseButton::Right => {
                        mouse.right = state;

                        match state {
                            ElementState::Pressed => {
                                figure.event(
                                    &mut render_state,
                                    &CanvasEvent::MouseRightPress(cursor.position),
                                );

                                mouse.right_press_start = cursor.position;
                                mouse.right_press_time = Instant::now();
                                window.set_cursor_icon(CursorIcon::Crosshair);
                            }
                            ElementState::Released => {
                                /*
                                figure.event(
                                    &mut draw_renderer,
                                    &CanvasEvent::MouseRightRelease(cursor.position),
                                );
                                */

                                if zoom_min <= mouse.right_press_start.dist(&cursor.position) {
                                    /*
                                    figure.event(
                                        &mut draw_renderer,
                                        &CanvasEvent::ZoomBounds(
                                            mouse.right_press_start, 
                                            cursor.position
                                        )
                                    );
                                    */
                                }
                                window.set_cursor_icon(CursorIcon::Default);
                            }
                        }
                    },
                    _ => {}
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved {
                    position,
                    ..
                },
                ..
            } => {
                //cursor.position = Point(position.x as f32, config.height as f32 - position.y as f32);

                if mouse.left == ElementState::Pressed 
                    && pan_min <= mouse.left_press_start.dist(&cursor.position) {
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

                    mouse.left_press_last = cursor.position;
                }
                if mouse.right == ElementState::Pressed
                    && pan_min <= mouse.left_press_start.dist(&cursor.position) {
                        /* 
                        figure.event(
                            &mut draw_renderer,
                            &CanvasEvent::MouseRightDrag(mouse.left_press_start, cursor.position),
                    );
                        */
                }
            }
            Event::RedrawRequested(_) => {
                //render_state.clear();
                //figure.draw(&mut figure_renderer);
                is_draw = true;
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }

        if is_draw || render_state.is_stale() {
            render_state.draw(&mut figure);
            // wgpu_canvas.render(&mut render_state, &mut figure);
            // render_main(device, queue, figure, canvas);

                /*
                figure_renderer.draw(
                    &mut figure,
                    (config.width, config.height),
                    window.scale_factor() as f32,
                    device, 
                    queue, 
                    view, 
                    encoder);
                    */
            //});
        }
    });
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
        encoder: &wgpu::CommandEncoder
    );
}

struct EventLoopArgs {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface,
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        println!("Hello, ui");
    }
}