use std::time::{Duration, Instant};

use essay_plot::api::Point;
use mind_ecs::TickConfig;
use winit::{
    event::{ElementState, Event, MouseButton, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    dpi::PhysicalPosition, keyboard::{Key, NamedKey},
};
use essay_ecs::prelude::*;

use super::ui_canvas::UiWindowEvent;

pub fn main_loop(mut app: App, tick_ms: Duration, ticks_per_cycle: usize) {
    // env_logger::init();

    let timer_length = tick_ms; // Duration::from_millis(100);

    let event_loop = app.remove_resource_non_send::<EventLoop<()>>().unwrap();

    //app.insert_resource(WinitEvents::default());
    let last_tick = Instant::now();
    let mut wait_until = Instant::now();
    let mut is_run = true;

    event_loop.run(move |event, window_target| {
        app.resource_mut::<WinitEvents>().clear();

        match event {
            Event::NewEvents(StartCause::Init) => {
                //wait_until = Instant::now() + timer_length;
            }
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                wait_until = Instant::now() + timer_length;

                let start = Instant::now();
                if is_run {
                    for _ in 0..ticks_per_cycle {
                        app.tick();
                    }
                }
            }
            Event::AboutToWait => {
                //wait_until = Instant::now() + timer_length;
                /*
                if is_run {
                    for _ in 0..ticks_per_cycle {
                        app.tick();
                    }
                }
                */
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { event, .. },
                ..
            } => {
                if event.state == ElementState::Pressed {
                    if let Key::Named(key)= event.logical_key {
                        match key {
                            NamedKey::Space => {
                                is_run = ! is_run;
                            },
                            _ => {
                            }
                        }
                    } else if let Key::Character(key)= event.logical_key {
                            match key.as_str() {
                            "1" => { 
                                app.resource_mut::<TickConfig>().set_n_ticks(1);
                                is_run = true;
                            },
                            "2" => { 
                                app.resource_mut::<TickConfig>().set_n_ticks(2);
                                is_run = true;
                            },
                            "3" => { 
                                app.resource_mut::<TickConfig>().set_n_ticks(4); 
                                is_run = true;
                            },
                            "4" => { 
                                app.resource_mut::<TickConfig>().set_n_ticks(8); 
                                is_run = true;
                            },
                            "5" => { 
                                app.resource_mut::<TickConfig>().set_n_ticks(16); 
                                is_run = true;
                            },
                            "6" => { 
                                app.resource_mut::<TickConfig>().set_n_ticks(32); 
                                is_run = true;
                            },
                            "7" => { 
                                app.resource_mut::<TickConfig>().set_n_ticks(64); 
                                is_run = true;
                            },
                            "8" => { 
                                app.resource_mut::<TickConfig>().set_n_ticks(128); 
                                is_run = true;
                            },
                            "9" => { 
                                app.resource_mut::<TickConfig>().set_n_ticks(1024); 
                                is_run = true;
                            },
                            "0" => { 
                                app.resource_mut::<TickConfig>().set_n_ticks(0); 
                                is_run = false;
                            },

                            " " => { 
                                if ! is_run {
                                    app.tick(); 
                                }
                                is_run = false;
                            },

                            "t" => { 
                                if ! is_run {
                                    app.tick(); 
                                }
                                is_run = false;
                            },
                            _ => {},
                        }
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                app.resource_mut::<Events<UiWindowEvent>>().send(UiWindowEvent::Resized(size.width, size.height));
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
            /*
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {

            },
            Event::RedrawEventsCleared => {
                window_target.set_control_flow(ControlFlow::WaitUntil(wait_until));
            },
            */
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => window_target.exit(), // window_target.set_control_flow(ControlFlow::Exit),
            _ => {}
        }
        window_target.set_control_flow(ControlFlow::WaitUntil(wait_until));
    }).unwrap();
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
