use frappe_gtk::prelude::*;
use gtk::prelude::*;

fn main() {
    gtk::init().unwrap();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("basic");
    window.set_border_width(20);

    let button = gtk::Button::with_label("Click me!");
    window.add(&button);

    button.stream_label(
        &button
            .clicked_events()
            .scan(0, |a, _| a + 1)
            .map(|n| format!("clicked {} times", n)),
    );

    window
        .delete_events(false)
        .observe_strong(|_| gtk::main_quit());

    window.show_all();
    gtk::main();
}
