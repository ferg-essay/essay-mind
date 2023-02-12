use eframe::{egui};

pub struct MainLoop {
    builder: MainLoopBuilder,
}

pub struct MainLoopBuilder {
    title: String,
    width: u32,
    height: u32,
}

impl MainLoopBuilder {
    pub fn build(self) -> MainLoop {
        MainLoop::new(self).expect("loop failed")
    }
}
impl MainLoop {
    pub fn new(builder: MainLoopBuilder) -> Result<Self, String> {
        Ok(Self {
            builder: builder,
        })
    }

    pub fn builder() -> MainLoopBuilder {
        MainLoopBuilder {
            title: String::from("Main Loop"),
            width: 800,
            height: 600,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(self.builder.width as f32, self.builder.height as f32)),
            ..Default::default()
        };

        let inner = AppInner {
            name: String::from(&self.builder.title),
        };

        eframe::run_native(
            &self.builder.title,
            options,
            Box::new(|_cc| Box::new(inner)),
        );

        Ok(())
    }
}

struct AppInner {
    name: String,
}

impl eframe::App for AppInner {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello GUI");
            ui.horizontal(|ui| {
                let name_label = ui.label("Name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.label(format!("Hello '{}'", self.name))
        });
    }
}
