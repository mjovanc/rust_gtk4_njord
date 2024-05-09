use std::path::Path;

use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, Button};
use njord::sqlite;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");

        let db_name = "../test.db";
        let db_path = Path::new(&db_name);

        match sqlite::open(db_path) {
            Ok(c) => {
                println!("Database opened successfully!");

                let task = Task {
                    id: 0,
                    title: String::from("john_doe"),
                    description: String::from("john@example.com"),
                    priority: String::from("123 Main St"),
                    status: String::from(""),
                };
            }
            Err(err) => eprintln!("Error opening the database: {}", err),
        }
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Task Manager")
        .child(&button)
        .build();

    // Present window
    window.present();
}
