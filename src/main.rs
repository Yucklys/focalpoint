use gtk::gio::ActionEntry;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4_layer_shell::{Layer, LayerShell, KeyboardMode};

const APP_ID: &str = "io.github.yucklys.focalpoint";

fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

		app.set_accels_for_action("win.close", &["Escape"]);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Focal Point")
        .default_width(350)
        .default_height(70)
        .build();

		window.init_layer_shell();
		window.set_layer(Layer::Top);
		window.set_keyboard_mode(KeyboardMode::Exclusive);

		let action_close = ActionEntry::builder("close")
				.activate(|window: &ApplicationWindow, _, _| {
						window.close();
				})
				.build();
		window.add_action_entries([action_close]);

    let label = gtk::Label::new(Some("Focal Point"));
    window.set_child(Some(&label));

    window.present();
}
