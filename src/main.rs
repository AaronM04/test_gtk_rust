extern crate gtk;
use gtk::prelude::*;
use gtk::{Window, WindowType, Button};

fn main() {

    gtk::init().unwrap();

    ///////////////// CREATE THINGS ////////////
    let window = Window::new(WindowType::Toplevel);
    let button = Button::new_with_label("I'm a button!");
    window.add(&button);

    window.show_all();         // Don't forget to make all widgets visible.

    ///////////////// ATTACH EVENTS ////////////
    // Handle clicking the button
    button.connect_clicked(|_| {
        println!("The button got clicked!");
    });
    // Handle closing of the window.
    window.connect_delete_event(|_, _| {
        // Stop the main loop.
        gtk::main_quit();
        // Let the default handler destroy the window.
        Inhibit(false)
    });

    ///////////////// RUN LOOP /////////////////
    gtk::main();
}
