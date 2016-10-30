extern crate gtk;
use gtk::prelude::*;
use gtk::{
    Window,
    WindowType,
    Button,
    Grid,
    DrawingArea,
    WindowPosition,
};

#[macro_use]
extern crate bitflags;

// TODO: see if I can use a complete list from somewhere
// List from https://www.roojs.com/seed/gir-1.2-gtk-3.0/gjs/Gdk.EventMask.html
bitflags! {
    flags GdkEventMask: i32 {
        const BUTTON1_MOTION_MASK =  32,
        const BUTTON_PRESS_MASK   = 256,
        const BUTTON_RELEASE_MASK = 512,
    }
}

const CELL_SIZE: u32 = 20;    // width/height of one cell in pixels

fn main() {

    gtk::init().unwrap();

    ///////////////// CREATE THINGS ////////////
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Test gtk rust");
    window.set_position(WindowPosition::Center);
    let grid = Grid::new();
    window.add(&grid);
    let button = Button::new_with_label("I'm a button!");
    button.set_hexpand(true);
    grid.attach(&button, 0, 0, 1, 1);    // pos<col,row>, size<col,row>
    let drawing_area = DrawingArea::new();
    drawing_area.set_size_request(200, 200);
    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);
    grid.attach(&drawing_area, 0, 1, 1, 1);
    drawing_area.set_events((BUTTON1_MOTION_MASK |
                             BUTTON_PRESS_MASK |
                             BUTTON_RELEASE_MASK).bits());

    ///////////////// SHOW EVERYTHING //////////
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

    drawing_area.connect_draw(|drawing_area, cr| {
        let width  = drawing_area.get_allocated_width();
        let height = drawing_area.get_allocated_height();
        if width != 200 || height != 200 {
            println!("width is {} and height is {}", width, height)
        }
        cr.set_source_rgb(1., 1., 1.);
        cr.rectangle(0.25, 0.25, 0.5, 0.5);
        cr.fill();
        Inhibit(false)
    });

    drawing_area.connect_button_press_event(|widget, evt| {
        println!("Mouse button pressed on drawing_area! evt is #{:?}", evt);
        Inhibit(false)
    });

    ///////////////// RUN LOOP /////////////////
    gtk::main();
}
