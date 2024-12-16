use eframe::{egui, NativeOptions};

fn _main(options: NativeOptions) {
    match eframe::run_native("Test app", options, Box::new(|_cc| Ok(Box::new(Application::new())))) {
		Ok(_) => log::info!("Finished with no issues"),
		Err(err) => { log::error!("Finished with an error: {err:?}"); panic!("Finished with an error: {:?}", err) }
	};
}

struct Application {
}

impl Application {
    pub fn new() -> Self {
		Self {}
	}
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.label("Some text here :D");
				if ui.button("Some button to see if the app is responding").clicked() {
					log::warn!("Button clicked");
				};
            });
        });
        ctx.request_repaint();
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
		log::error!("Saving!");
    }

    fn on_exit(&mut self) {
		log::error!("Exiting!");
    }
}


#[cfg(any(target_os = "ios", target_os = "android"))]
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
            std::process::abort()
        }
    }
}

#[cfg(target_os = "ios")]
fn _start_app() {
    stop_unwind(|| main());
}

#[no_mangle]
#[inline(never)]
#[cfg(target_os = "ios")]
pub extern "C" fn start_app() {
    _start_app();
}

#[cfg(not(target_os = "android"))]
pub fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .parse_default_env()
        .init();

    _main(NativeOptions::default());
}

#[allow(dead_code)]
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Warn),
    );

    let mut opts = NativeOptions::default();
	opts.android_app = Some(app);
	let default_path = format!("/storage/emulated/0/Android/data/io.hellopaint.egui.egui_mobile_test/files/save.ron");
    opts.persistence_path = Some(default_path.into());

    stop_unwind(|| _main(opts));
}
