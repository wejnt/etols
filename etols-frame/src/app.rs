use egui::{
    style::Margin, widgets, CentralPanel, Context, FontData, FontDefinitions, FontFamily, Label,
    ScrollArea, Sense, SidePanel, TopBottomPanel,
};
use epi::{App, Frame};

pub struct EtolsApp {
    title: String,
    text: String,
    show: bool,
}

impl EtolsApp {
    pub fn new(title: &str) -> EtolsApp {
        EtolsApp {
            title: title.to_owned(),
            text: Default::default(),
            show: false,
        }
    }
    pub fn show(&mut self) {
        self.show = !self.show;
    }
    fn editor_ui(&mut self, ui: &mut egui::Ui) {
        let Self { text, .. } = self;
        ui.add(
            egui::TextEdit::multiline(text)
                .font(egui::TextStyle::Monospace) // for cursor height
                .code_editor()
                .desired_rows(1)
                .lock_focus(true)
                .desired_width(f32::INFINITY),
        );
    }
}

impl App for EtolsApp {
    fn update(&mut self, ctx: &Context, _frame: &Frame) {
        TopBottomPanel::top("top")
            // .frame(egui::Frame::default().margin(Margin::same(0.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    widgets::global_dark_light_mode_switch(ui);
                });
            });

        let left_panel;

        if !self.show {
            left_panel = SidePanel::left("left_panel_close")
                .resizable(false)
                .width_range(0.0..=12.0);
        } else {
            left_panel = SidePanel::left("left_panel_open")
                // .frame(egui::Frame::fill(self, fill))
                .resizable(true)
                .max_width(120.00)
                .width_range(0.0..=300.0);
        }
        left_panel.show(ctx, |ui| {
            let close_icon = Label::new("⮨").sense(Sense::click());
            let open_icon = Label::new("⮫").sense(Sense::click());
            if self.show {
                TopBottomPanel::top("close_panel")
                    .frame(egui::Frame::none().margin(Margin::same(0.0)))
                    .show_inside(ui, |ui| {
                        SidePanel::left("file")
                            .resizable(false)
                            .frame(egui::Frame::none().margin(Margin::same(0.0)))
                            .show_inside(ui, |ui| {
                                ui.selectable_label(true, "📄·README00000000000.md")
                                    .clicked();
                            });
                        SidePanel::right("close")
                            .resizable(false)
                            .frame(egui::Frame::none().margin(Margin::same(0.0)))
                            .width_range(0.0..=12.0)
                            .show_inside(ui, |ui| {
                                if ui.add(close_icon).clicked() {
                                    self.show();
                                }
                            });
                    });
                ui.selectable_label(false, "text").clicked();
            } else {
                if ui.add(open_icon).clicked() {
                    self.show();
                }
            }
        });
        CentralPanel::default()
            .show(ctx, |ui| {
                ScrollArea::vertical()
                    .id_source("source")
                    .show(ui, |ui| self.editor_ui(ui));
            });
    }

    fn name(&self) -> &str {
        &self.title
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &Frame, _storage: Option<&dyn epi::Storage>) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "jetbrainsmono-regular".to_owned(),
            FontData::from_static(include_bytes!("../../fonts/JetBrainsMono-Regular.ttf")),
        );
        fonts.font_data.insert(
            "sarasa-mono-sc-nerd-regular".to_owned(),
            FontData::from_static(include_bytes!(
                "../../fonts/sarasa-mono-sc-nerd-regular.ttf"
            )),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "jetbrainsmono-regular".to_owned());

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(1, "sarasa-mono-sc-nerd-regular".to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "jetbrainsmono-regular".to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(1, "sarasa-mono-sc-nerd-regular".to_owned());

        ctx.set_fonts(fonts);
    }
}
