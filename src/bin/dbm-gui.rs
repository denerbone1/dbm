use eframe::egui;
use dbm::convert_expression;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([760.0, 520.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Denerbone's Math",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(DbmApp::default()))
        }),
    )
}

#[derive(Default)]
struct DbmApp {
    expression: String,
    settings: AppSettings,
    show_settings: bool,
}

#[derive(Clone, Copy, PartialEq, Default)]
enum AppTheme {
    #[default]
    Dark,
    Light,
    Violet,
}

struct AppSettings {
    theme: AppTheme,
    font_size: f32,
    show_full_result: bool,
    live_calculation: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: AppTheme::Dark,
            font_size: 16.0,
            show_full_result: true,
            live_calculation: true,
        }
    }
}

impl eframe::App for DbmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let calculation = if self.expression.trim().is_empty() {
            None
        } else {
            Some(convert_expression(&self.expression))
        };

        let visuals = match self.settings.theme {
            AppTheme::Dark => egui::Visuals::dark(),
            AppTheme::Light => egui::Visuals::light(),
            AppTheme::Violet => {
                let mut visuals = egui::Visuals::dark();

                visuals.override_text_color = Some(
                    egui::Color32::from_rgb(225, 215, 255),
                );

                visuals.selection.bg_fill =
                    egui::Color32::from_rgb(115, 75, 180);

                visuals
            }
        };

        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();

        for text_style in [
            egui::TextStyle::Body,
            egui::TextStyle::Button,
            egui::TextStyle::Heading,
            egui::TextStyle::Monospace,
            egui::TextStyle::Small,
        ] {
            if let Some(font_id) = style.text_styles.get_mut(&text_style) {
                font_id.size = self.settings.font_size;
            }
        }

        ctx.set_style(style);

        if self.show_settings {
            egui::Window::new("Settings")
                .open(&mut self.show_settings)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.heading("Appearance");

                    ui.horizontal(|ui| {
                        ui.label("Theme:");

                        egui::ComboBox::from_id_salt("theme")
                            .selected_text(match self.settings.theme {
                                AppTheme::Dark => "Dark",
                                AppTheme::Light => "Light",
                                AppTheme::Violet => "Violet",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.settings.theme,
                                    AppTheme::Dark,
                                    "Dark",
                                );

                                ui.selectable_value(
                                    &mut self.settings.theme,
                                    AppTheme::Light,
                                    "Light",
                                );

                                ui.selectable_value(
                                    &mut self.settings.theme,
                                    AppTheme::Violet,
                                    "Violet",
                                );
                            });
                    });

                    ui.add(
                        egui::Slider::new(
                            &mut self.settings.font_size,
                            12.0..=24.0,
                        )
                            .text("Font size"),
                    );

                    ui.separator();

                    ui.heading("Calculator");

                    ui.checkbox(
                        &mut self.settings.live_calculation,
                        "Live calculation",
                    );

                    ui.checkbox(
                        &mut self.settings.show_full_result,
                        "Show full result",
                    );
                });
        }

        // Верхняя панель
        egui::TopBottomPanel::top("header")
            .exact_height(55.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if ui.button("⚙ Settings").clicked() {
                        self.show_settings = true;
                    }
                    ui.heading("Denerbone's Math");
                    ui.separator();
                    ui.label("DBM Calculator");
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.label("v1.0");
                        },
                    );
                });
            });

        // Боковая панель
        egui::SidePanel::left("syntax")
            .exact_width(230.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Syntax");
                ui.add_space(12.0);

                ui.label("Operators");
                ui.separator();

                ui.monospace("~     Addition");
                ui.monospace("|||   Subtraction");
                ui.monospace("##    Power");
                ui.monospace("V_    Square root");

                ui.add_space(16.0);

                ui.label("Numbers");
                ui.separator();

                ui.monospace("|-1-1| = 2");
                ui.monospace("|-2-1| = 3");
                ui.monospace("|();5| = 0.5");

                ui.add_space(16.0);

                ui.label("Grouping");
                ui.separator();

                ui.monospace("{...}");
                ui.label("Expression grouping");

                ui.add_space(8.0);
                ui.monospace("()");
                ui.label("Zero");
            });

        // Основная рабочая область
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(12.0);

            ui.heading("Calculator");
            ui.label("Enter a DBM expression");

            ui.add_space(12.0);

            ui.add(
                egui::TextEdit::multiline(&mut self.expression)
                    .font(egui::TextStyle::Monospace)
                    .desired_rows(3)
                    .desired_width(f32::INFINITY)
                    .hint_text("2~3"),
            );

            ui.add_space(24.0);

            ui.heading("Result");
            ui.separator();

            match calculation {
                Some(Ok(result)) => {
                    ui.colored_label(
                        egui::Color32::LIGHT_GREEN,
                        "Calculation successful",
                    );

                    ui.add_space(8.0);

                    ui.label(result);
                }

                Some(Err(error)) => {
                    ui.colored_label(
                        egui::Color32::LIGHT_RED,
                        "Calculation error",
                    );

                    ui.add_space(8.0);

                    ui.label(error);
                }

                None => {
                    ui.label("Waiting for expression...");
                }
            }

            ui.with_layout(
                egui::Layout::bottom_up(egui::Align::LEFT),
                |ui| {
                    ui.separator();
                    ui.small("Denerbone's Math | Powered by egui");
                },
            );
        });
    }
}