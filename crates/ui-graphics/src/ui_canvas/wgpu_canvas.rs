use wgpu::{SurfaceTexture, TextureView};
use winit::{window::{Window, CursorIcon}, event_loop::EventLoop};


pub(crate) struct WgpuCanvas {
    pub(crate) device: wgpu::Device,
    pub(crate) surface: wgpu::Surface,
    pub(crate) queue: wgpu::Queue,
    pub(crate) config: wgpu::SurfaceConfiguration,

    pub(crate) _window: winit::window::Window,

    //event_loop: Option<EventLoop<()>>,
}

impl WgpuCanvas {
    pub fn new(event_loop: &EventLoop<()>) -> WgpuCanvas {
        // let event_loop = EventLoop::new();
        let window = winit::window::Window::new(&event_loop).unwrap();

        let wgpu_canvas = pollster::block_on(init_wgpu_args(window));

        // wgpu_canvas.event_loop = Some(event_loop);

        wgpu_canvas
    }

    pub fn _draw(
        &mut self, 
        draw: impl FnOnce(&WgpuCanvas, &wgpu::TextureView)
    ) {
        let frame = self.surface
            .get_current_texture()
            .expect("Failed to get next swap chain texture");

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        //self.clear_screen(&view);

        draw(self, &view);

        frame.present();
    }

    pub fn create_view(&mut self) -> CanvasView {
        let frame = self.surface
            .get_current_texture()
            .expect("Failed to get next swap chain texture");

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        CanvasView {
            frame,
            view
        }
    }

    pub(crate) fn clear_screen(&self, view: &wgpu::TextureView) {

        let mut encoder =
            self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
        // clip 
        // rpass.set_viewport(0., 0., 1., 1., 0.0, 1.0);
    
            let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    }
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.queue.submit(Some(encoder.finish()));
    }

    pub(crate) fn window_bounds(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }
}

pub struct CanvasView {
    frame: SurfaceTexture,
    pub(crate) view: TextureView,
}

impl CanvasView {
    pub(crate) fn flush(self) {
        self.frame.present();
    }
}

async fn init_wgpu_args(window: Window) -> WgpuCanvas {
    window.set_title("Essays on Vertebrate Mind");
    window.set_cursor_icon(CursorIcon::Default);

    let size = window.inner_size();

    let instance = wgpu::Instance::default();

    let surface = unsafe { instance.create_surface(&window) }.unwrap();


    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let texture_format = swapchain_capabilities.formats[0];

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: texture_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    WgpuCanvas {
        device,
        surface,
        queue,
        config,
        _window: window,
    }
}
