use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    name: String,
    clicks: u32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.heading("Rust + egui starter");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Welcome to your desktop app.");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Your name:");
                ui.text_edit_singleline(&mut self.name);
            });

            if ui.button("Click me").clicked() {
                self.clicks += 1;
            }

            ui.label(format!("Clicks: {}", self.clicks));

            if !self.name.is_empty() {
                ui.heading(format!("Hello, {}!", self.name));
            }
        });
    }
}