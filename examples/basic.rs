use frappe_gtk::prelude::*;
use gtk;
use gtk::prelude::*;

fn main() {
    gtk::init().unwrap();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("basic");
    window.set_border_width(20);

    let button = gtk::Button::new_with_label("Click me!");
    window.add(&button);

    let clicked = button.clicked_events();
    let counter = clicked
        .fold(0, |a, _| a + 1)
        .snapshot(&clicked, |n, _| format!("clicked {} times", n));
    button.stream_label(&counter);

    let _del = window.delete_events(false).inspect(|_| gtk::main_quit());

    window.show_all();
    gtk::main();
}
