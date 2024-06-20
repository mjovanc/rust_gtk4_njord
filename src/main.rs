use std::collections::HashMap;
use std::path::Path;

use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, Box, Button, Entry, Label, ListBox, ListBoxRow, Orientation};
use gtk::glib::clone;
use njord::sqlite;

use crate::schema::Task;

mod schema;
mod status;

const APP_ID: &str = "org.gtk_rs.TaskManager";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a vertical box container
    let vbox = Box::new(Orientation::Vertical, 5);

    // Create input fields for new tasks
    let title_entry = Entry::builder().placeholder_text("Title").build();
    let description_entry = Entry::builder().placeholder_text("Description").build();
    let priority_entry = Entry::builder().placeholder_text("Priority").build();
    let status_entry = Entry::builder().placeholder_text("Status").build();

    // Create a button to add new tasks
    let add_button = Button::builder().label("Add Task").build();

    // Create a ListBox to display tasks
    let task_list = ListBox::new();

    // Connect the "clicked" signal of the add button
    add_button.connect_clicked(clone!(@weak task_list, @weak title_entry, @weak description_entry, @weak priority_entry, @weak status_entry => move |_| {
        let db_name = "test.db";
        let db_path = Path::new(&db_name);

        if let Ok(conn) = sqlite::open(db_path) {
            let task = Task {
                id: 0, // Assuming id is auto-incremented
                title: title_entry.text().to_string(),
                description: description_entry.text().to_string(),
                priority: priority_entry.text().to_string(),
                status: status_entry.text().to_string(),
            };

            if let Ok(_) = sqlite::insert(conn, vec![task.clone()]) {
                // Create a new row in the task list
                add_task_to_list(&task_list, &task);
            } else {
                eprintln!("Failed to insert task into database.");
            }
        } else {
            eprintln!("Failed to open database.");
        }
    }));

    // Add the input fields and button to the vbox
    vbox.append(&title_entry);
    vbox.append(&description_entry);
    vbox.append(&priority_entry);
    vbox.append(&status_entry);
    vbox.append(&add_button);
    vbox.append(&task_list);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Task Manager")
        .child(&vbox)
        .build();

    // Load tasks from database and display them
    load_tasks(&task_list);

    // Present window
    window.present();
}

fn add_task_to_list(task_list: &ListBox, task: &Task) {
    let row = ListBoxRow::new();
    let row_box = Box::new(Orientation::Horizontal, 5);

    let title_label = Label::new(Some(&task.title));
    let description_label = Label::new(Some(&task.description));
    let priority_label = Label::new(Some(&task.priority));
    let status_label = Label::new(Some(&task.status));

    row_box.append(&title_label);
    row_box.append(&description_label);
    row_box.append(&priority_label);
    row_box.append(&status_label);
    row.set_child(Some(&row_box));

    // Connect row click event to show task details
    row.connect_activate(clone!(@strong task => move |_| {
        show_task_details(&task);
    }));

    task_list.append(&row);
    task_list.show();
}

fn load_tasks(task_list: &ListBox) {
    let db_name = "test.db";
    let db_path = Path::new(&db_name);

    if let Ok(conn) = sqlite::open(db_path) {
        let columns = vec!["id".to_string(), "title".to_string(), "description".to_string(), "priority".to_string(), "status".to_string()];

        let result = sqlite::select(conn, columns)
            .from(Task::default())
            .build();

        match result {
            Ok(tasks) => {
                for task in tasks {
                    add_task_to_list(task_list, &task);
                }
            }
            Err(error) => eprintln!("Failed to SELECT: {:?}", error),
        }
    } else {
        eprintln!("Failed to open database.");
    }
}

fn show_task_details(task: &Task) {
    // Create a new window to display task details
    let detail_window = ApplicationWindow::builder()
        .title("Task Details")
        .default_width(400)
        .default_height(300)
        .build();

    let detail_vbox = Box::new(Orientation::Vertical, 5);
    detail_vbox.append(&Label::new(Some(&format!("Title: {}", task.title))));
    detail_vbox.append(&Label::new(Some(&format!("Description: {}", task.description))));
    detail_vbox.append(&Label::new(Some(&format!("Priority: {}", task.priority))));
    detail_vbox.append(&Label::new(Some(&format!("Status: {}", task.status))));

    detail_window.set_child(Some(&detail_vbox));
    detail_window.present();
}
