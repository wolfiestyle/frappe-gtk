use frappe_gtk::prelude::*;
use gtk;
use gtk::prelude::*;
use std::thread;
use std::time::Duration;

macro_rules! with {
    ($obj:expr, |$arg:ident| $body:expr) => {{
        let $arg = $obj;
        $body;
        $arg
    }};
}

fn main() {
    gtk::init().unwrap();

    // build the Gtk UI
    let (listbox, cmd_add, cmd_del, lbl_count, spinner);
    let window = with!(gtk::Window::new(gtk::WindowType::Toplevel), |window| {
        window.set_default_size(300, 300);
        window.set_border_width(10);
        window.set_title("gtktest");

        window.add(&with!(
            gtk::Box::new(gtk::Orientation::Vertical, 5),
            |container| {
                container.add(&with!(gtk::Overlay::new(), |row| {
                    lbl_count = gtk::Label::new("counter: 0");
                    row.add(&lbl_count);

                    spinner = gtk::Spinner::new();
                    spinner.set_halign(gtk::Align::Start);
                    row.add_overlay(&spinner);
                }));

                container.add(&with!(gtk::ScrolledWindow::new(None, None), |scrolled| {
                    scrolled.set_vexpand(true);

                    listbox = gtk::ListBox::new();
                    scrolled.add(&listbox);
                }));

                container.add(&with!(
                    gtk::Box::new(gtk::Orientation::Horizontal, 3),
                    |row| {
                        cmd_add = gtk::Button::new_with_label("Add item delayed");
                        row.add(&cmd_add);

                        cmd_del = gtk::Button::new_with_label("Remove item");
                        row.add(&cmd_del);
                    }
                ));
            }
        ));
    });

    // handle the "Add item" button events
    let add_clicked = cmd_add.clicked_events();
    let counter = add_clicked
        .fold(0, |a, _| a + 1)
        .snapshot(&add_clicked, |a, _| a);
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
    let job_inc = counter.merge_with(&counter_worker, |_| 1, |_| -1);
    let job_count = job_inc.fold(0, |a, n| a + *n);
    let spinner_active = job_count.snapshot(&job_inc, |a, _| a != 0);
    // update the UI state using the data from the streams
    let counter_labels = counter_worker.map(|s| gtk::Label::new(s.as_str()).show_());
    lbl_count.stream_label(&counter_str);
    listbox.stream_add(&counter_labels);
    spinner.stream_active(&spinner_active);

    // handle the "Remove item" button events
    let del_clicked = cmd_del.clicked_events();
    let deleted = gtk_lift!(listbox.get_selected_row)
        .wrap_opt_fragile()
        .snapshot(&del_clicked, |a, _| a)
        .filter_some()
        .unwrap_fragile();
    listbox.stream_remove(&deleted);

    // handle the window close event
    let win_ = window.wrap_fragile();
    let _close_ev = window
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
        .inspect(|_| gtk::main_quit());

    window.show_all();
    gtk::main();
}
