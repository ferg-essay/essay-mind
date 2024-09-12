use std::{sync::{Arc, Mutex}, time::{Duration, Instant}};

use mind_ecs::TickConfig;
use winit::{
    dpi::PhysicalPosition, event::{ElementState, Event, MouseButton, StartCause, WindowEvent}, event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget}, keyboard::{Key, NamedKey}
};
use essay_ecs::{prelude::*, core::error::{Error, Result}};

use super::ui_canvas::UiWindowEvent;

pub fn main_loop(mut app: App, tick_ms: Duration, ticks_per_cycle: usize) -> Result<()> {
    // env_logger::init();

    let timer_length = tick_ms; // Duration::from_millis(100);

    let event_loop = app.remove_resource_non_send::<EventLoop<()>>().unwrap();

    let mut wait_until = Instant::now();
    let mut is_run = true;

    let result_handle = Arc::new(Mutex::new(ResultHandle::default()));
    let mut result_inner = result_handle.clone();

    event_loop.run(move |event, window_target| {
        if window_target.exiting() {
            return;
        }
        
        app.resource_mut::<WinitEvents>().clear();

        match event {
            Event::NewEvents(StartCause::Init) => {
            }
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                wait_until = Instant::now() + timer_length;

                if is_run {
                    for _ in 0..ticks_per_cycle {
                        win_tick(&mut app, &mut result_inner, window_target);
                    }
                }
            }
            Event::AboutToWait => {
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
                                    win_tick(&mut app, &result_inner, &window_target);
                                }
                                is_run = false;
                            },

                            "t" => { 
                                if ! is_run {
                                    win_tick(&mut app, &result_inner, &window_target);
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
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => window_target.exit(), // window_target.set_control_flow(ControlFlow::Exit),
            _ => {}
        }
        window_target.set_control_flow(ControlFlow::WaitUntil(wait_until));
    }).unwrap();

    let err = result_handle.lock().unwrap().err.take();

    if let Some(err) = err {
        Err(err)
    } else {
        Ok(())
    }
}

fn win_tick(
    app: &mut App, 
    result_inner: &Mutex<ResultHandle>, 
    window_target: &EventLoopWindowTarget<()>) {
    app.tick().unwrap_or_else(|err| {
        result_inner.lock().unwrap().err = Some(err);
        window_target.exit();
    });
}

#[derive(Default)]
struct ResultHandle {
    err: Option<Error>
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
            }
            MouseButton::Right => {
                match state {
                    ElementState::Pressed => {
                    }
                    ElementState::Released => {
                    }
                }
            }
            _ => {}
        }
    }

    fn cursor_event(&mut self, _position: PhysicalPosition<f64>) {
    }
}
