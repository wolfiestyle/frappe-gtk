use frappe_gtk::prelude::*;
use gtk;
use gtk::prelude::*;
use std::thread;
use std::time::Duration;
use with_macro::with;

fn main() {
    gtk::init().unwrap();

    // build the Gtk UI
    let (listbox, cmd_add, cmd_del, lbl_count, spinner);
    let window = with! {
        gtk::Window::new(gtk::WindowType::Toplevel) =>
            .set_default_size(300, 300)
            .set_border_width(10)
            .set_title("listbox")
            .add(&with!{
                gtk::Box::new(gtk::Orientation::Vertical, 5) =>
                    .add(&with!{
                        gtk::Overlay::new() =>
                            lbl_count = gtk::Label::new("counter: 0");
                            .add(&lbl_count)

                            spinner = with!{
                                gtk::Spinner::new() =>
                                    .set_halign(gtk::Align::Start)
                            };
                            .add_overlay(&spinner)
                    })
                    .add(&with!{
                        gtk::ScrolledWindow::new(None, None) =>
                            .set_vexpand(true)

                            listbox = gtk::ListBox::new();
                            .add(&listbox)
                    })
                    .add(&with!{
                        gtk::Box::new(gtk::Orientation::Horizontal, 3) =>
                            cmd_add = gtk::Button::new_with_label("Add item delayed");
                            .add(&cmd_add)

                            cmd_del = gtk::Button::new_with_label("Remove item");
                            .add(&cmd_del)
                    })
            })
    };

    // handle the "Add item" button events
    let counter = cmd_add.clicked_events().scan(0, |a, _| a + 1);
    let counter_str = counter.map(|n| format!("counter: {}", n));
    // do some work on another thread in response to this event
    let counter_worker = counter_str
        .map_n(|val, sender| {
            let val = val.into_owned();
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(1000));
                sender.send(val);
            });
        })
        .to_main_thread();
    // count the pending jobs and use it to display a spinner
    let spinner_active = counter
        .merge_with(&counter_worker, |_| 1, |_| -1)
        .scan(0, |a, n| a + *n)
        .map(|a| *a != 0);
    // update the UI state using the data from the streams
    lbl_count.stream_label(&counter_str);
    listbox.stream_add(&counter_worker.map(|s| gtk::Label::new(s.as_str()).show_()));
    spinner.stream_active(&spinner_active);

    // handle the "Remove item" button events
    let del_clicked = cmd_del.clicked_events();
    let deleted = gtk_lift!(listbox.get_selected_row)
        .snapshot(&del_clicked, |a, _| a)
        .filter_some();
    listbox.stream_remove(&deleted);

    // handle the window close event
    let win_ = window.wrap_fragile();
    window
        .delete_events(true)
        .map(move |_| {
            gtk::MessageDialog::new(
                Some(win_.get()),
                gtk::DialogFlags::MODAL,
                gtk::MessageType::Warning,
                gtk::ButtonsType::YesNo,
                "Are you sure you want to quit?",
            )
        })
        .run_dialog()
        .filter(|resp| *resp == gtk::ResponseType::Yes.into())
        .observe_strong(|_| gtk::main_quit());

    window.show_all();
    gtk::main();
}
