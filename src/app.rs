use egui_plot::{Legend, Line, Plot, PlotPoints};
use egui::Color32;

const E_E: f64 = 55.0;
const E_I: f64 = -70.0;
const E_L: f64 = -70.0;


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    excitation: f64,
    #[serde(skip)]
    delta_time: f64,
    #[serde(skip)]
    vm: f64,
    #[serde(skip)]
    tick: i32,
    #[serde(skip)]
    inhibition: f64,
    #[serde(skip)]
    history: Vec<[f64; 2]>,
    #[serde(skip)]
    net_history: Vec<[f64; 2]>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
	    excitation: 0.0,
	    delta_time: 0.0,
	    vm: -70.0, // resting potential
            tick: 0,
            inhibition: 0.0,
            history: vec![],
            net_history: vec![],
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

	egui::SidePanel::left("left").resizable(true).show(ctx, |ui| {
            ui.label("Constants");
	    ui.label(format!("Excitatory channel E_e: {}", E_E));
	    ui.label(format!("Inhibitory channel E_i: {}", E_I));
	    ui.label(format!("Leak channel E_l: {}", E_L));
	    ui.separator();

            ui.label(format!("g_e: {}", self.excitation));
            ui.label(format!("Delta time: {}", self.delta_time));
            ui.separator();

	    ui.add(egui::DragValue::new(&mut self.excitation).prefix("Excitation: ").clamp_range(0.0..=1.0).speed(0.1));
	    ui.add(egui::DragValue::new(&mut self.delta_time).prefix("Dt: ").clamp_range(0.0..=1.0).speed(0.1));
	    ui.separator();

            if ui.button("Reset").clicked() {
                self.history.clear();
                self.net_history.clear();
                self.vm = -70.0;
                self.excitation = 0.0;
                self.delta_time = 0.0;
                self.tick = 0;
                self.inhibition = 0.0;

            }
            if ui.button("Next tick").clicked() {
                let i_net = compute_net_current(self.excitation, self.inhibition, self.vm);
                let vm = compute_vm(self.vm, self.delta_time, i_net);
                self.vm = vm;
                self.history.push([self.tick as f64, vm]);
                self.net_history.push([self.tick as f64, i_net]);
                self.tick += 1;
            }
            ui.separator();

            let s = if self.vm > E_I { format!("Vm: {}", self.vm) } else { format!("Vm at resting potential: {}", self.vm) };
	    ui.label(s);
	    
	});

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("graph");

	    plot(ui, &self.history, &self.net_history);
        });
    }
}

const G_L: f64 = 1.0;
const G_BAR_E: f64 = 0.5;
const G_BAR_I: f64 = 0.5;
const G_BAR_L: f64 = 1.0;
const LEAK: f64 = G_L * G_BAR_L;

fn compute_net_current(g_e: f64, g_i: f64, vm: f64) -> f64 {
    g_e * G_BAR_E * (E_E - vm) +
        g_i * G_BAR_I * (E_I - vm) +
        LEAK * (E_L - vm)
}

fn compute_vm(prev_vm: f64, dt: f64, i_net: f64) -> f64 {
    prev_vm + dt * i_net
}

fn plot(ui: &mut egui::Ui, vm: &Vec<[f64; 2]>, net_hist: &Vec<[f64; 2]>) -> egui::Response {
        let graph: Vec<[f64; 2]> = vm
        .iter()
        .map(|&[x, y]| [x, y])
        .collect();

    let net_graph: Vec<[f64; 2]> = net_hist
        .iter()
        .map(|&[x, y]| [x, y])
        .collect();
    let my_plot = Plot::new("My Plot").legend(Legend::default());
    let inner = my_plot.show(ui, |plot_ui| {
        plot_ui.line(Line::new(PlotPoints::from(graph)).name("V_m"));
        plot_ui.line(Line::new(PlotPoints::from(net_graph)).color(Color32::LIGHT_BLUE).name("I_net"));
    });
    inner.response
}
