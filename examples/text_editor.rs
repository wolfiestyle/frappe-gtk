use frappe::{Signal, Stream};
use frappe_gtk::prelude::*;
use gtk::prelude::*;
use std::path::PathBuf;
use with_macro::with;

fn main() {
    gtk::init().unwrap();

    let (mnu_new, mnu_open, mnu_save, mnu_saveas, mnu_quit, mnu_about);
    let (notebook, textview);

    let window = with! {
        gtk::Window::new(gtk::WindowType::Toplevel) =>
            .set_title("Text editor")
            .set_default_size(640, 480)
            .add(&with!{
                gtk::Box::new(gtk::Orientation::Vertical, 0) =>
                    .add(&with!{
                        gtk::MenuBar::new() =>
                            .append(&with!{
                                gtk::MenuItem::with_label("File") =>
                                    .set_submenu(Some(&with!{
                                        gtk::Menu::new() =>
                                            mnu_new = gtk::MenuItem::with_label("New");
                                            .append(&mnu_new)
                                            mnu_open = gtk::MenuItem::with_label("Open...");
                                            .append(&mnu_open)
                                            mnu_save = gtk::MenuItem::with_label("Save");
                                            .append(&mnu_save)
                                            mnu_saveas = gtk::MenuItem::with_label("Save as...");
                                            .append(&mnu_saveas)
                                            .append(&gtk::SeparatorMenuItem::new())
                                            mnu_quit = gtk::MenuItem::with_label("Quit");
                                            .append(&mnu_quit)
                                    }))
                            })
                            .append(&with!{
                                gtk::MenuItem::with_label("Help") =>
                                    .set_submenu(Some(&with!{
                                        gtk::Menu::new() =>
                                            mnu_about = gtk::MenuItem::with_label("About");
                                            .append(&mnu_about)
                                    }))
                            })
                    })
                    notebook = with!{
                        gtk::Notebook::new() =>
                            textview = gtk::TextView::new();
                            .add(&textview)
                    };
                    .pack_start(&notebook, true, true, 0)
            })
    };

    let new_ev = mnu_new
        .activate_events()
        .map(|_| gtk::TextView::new().show_());
    notebook.stream_add(&new_ev);

    let buffer = textview.buffer().unwrap();
    let buf_modified = gtk_lift!(buffer.is_modified);

    let _filename = Signal::cyclic(|name| {
        let new_ev = mnu_new.activate_events();
        let name_s1 = confirm_unsaved(&new_ev, &buf_modified, name.clone(), &window);
        buffer.stream_text(&name_s1.map(|_| String::new()));
        buffer.stream_modified(&name_s1.map(|_| false));

        let open_ev = mnu_open.activate_events();
        let name_s2 = open_file(&open_ev, &buf_modified, name.clone(), &window);

        let save_ev = mnu_save.activate_events();
        let name_s3 = save_file(&save_ev, &name, &window);

        let saveas_ev = mnu_saveas.activate_events();
        let name_s4 = save_file(&saveas_ev, &Default::default(), &window);

        let quit_ev = mnu_quit
            .activate_events()
            .merge(&window.delete_events(true));
        let name_s5 = confirm_unsaved(&quit_ev, &buf_modified, name.clone(), &window);
        name_s5.observe(|_| gtk::main_quit());

        name_s1
            .merge(&name_s2)
            .merge(&name_s3)
            .merge(&name_s4)
            .merge(&name_s5)
            .hold(None)
    });

    window.show_all();
    gtk::main();
}

#[derive(Clone)]
enum SaveResponse {
    Unmodified,
    DontSave,
    SaveAs(gtk::FileChooserDialog),
    Save(PathBuf),
}

fn confirm_unsaved(
    trigger: &Stream<()>,
    modified: &Signal<bool>,
    filename: Signal<Option<PathBuf>>,
    win: &gtk::Window,
) -> Stream<Option<PathBuf>> {
    let win_ = win.wrap_fragile();
    let win2 = win_.clone();
    modified
        // check if unsaved changes
        .snapshot(trigger, |a, _| a)
        // show save prompt if unsaved
        .map(move |modif| {
            if *modif {
                let dlg = gtk::MessageDialog::new(
                    Some(win_.get()),
                    gtk::DialogFlags::MODAL,
                    gtk::MessageType::Warning,
                    gtk::ButtonsType::YesNo,
                    "There are unsaved changes. Do you want to save?",
                );
                dlg.add_button("Cancel", gtk::ResponseType::Cancel);
                Some(dlg)
            } else {
                None
            }
        })
        // run dialog if unsaved (Option<Dialog>)
        .run_opt_dialog()
        // remove cancel (Option<i32>: Some = dialog response, None = unmodified)
        .filter(|optresp| {
            optresp
                .map(|resp| resp != gtk::ResponseType::Cancel)
                .unwrap_or(true)
        })
        // check dialog response
        .map(move |optresp| {
            if let Some(resp) = *optresp {
                if resp == gtk::ResponseType::Yes {
                    if let Some(name) = filename.sample() {
                        SaveResponse::Save(name)
                    } else {
                        let dlg = gtk::FileChooserDialog::with_buttons(
                            Some("Save file as"),
                            Some(win2.get()),
                            gtk::FileChooserAction::Save,
                            &[
                                ("_Cancel", gtk::ResponseType::Cancel),
                                ("_Save", gtk::ResponseType::Accept),
                            ],
                        );
                        SaveResponse::SaveAs(dlg)
                    }
                } else if resp == gtk::ResponseType::No {
                    SaveResponse::DontSave
                } else {
                    unreachable!()
                }
            } else {
                SaveResponse::Unmodified
            }
        })
        // run the SaveResponse::SaveAs dialog to obtain a filename
        .map_n(|saveresp, sender| {
            match saveresp.into_owned() {
                SaveResponse::SaveAs(dialog) => {
                    dialog.show();
                    dialog.connect_response(move |dlg, resp| {
                        let fname = dlg.filename();
                        dlg.hide();
                        if resp == gtk::ResponseType::Accept {
                            println!("got filename {:?}", fname);
                            if let Some(name) = fname {
                                sender.send(SaveResponse::Save(name));
                            }
                        }
                        // cancel gets removed from the stream
                    });
                }
                other => sender.send(other),
            }
        })
        // save the file
        .map(|saveresp| {
            if let SaveResponse::Save(fname) = saveresp.into_owned() {
                println!("saving file: {:?}", fname);
            //Some(fname)
            } else {
                // Unmodified/DontSave = discard buffer
                println!("discarding changes");
                //None
            }
            // we're replacing the buffer so clear the filename
            None
        })
}

fn open_file(
    trigger: &Stream<()>,
    modified: &Signal<bool>,
    filename: Signal<Option<PathBuf>>,
    win: &gtk::Window,
) -> Stream<Option<PathBuf>> {
    eprintln!("open_file: unimplemented");
    Default::default()
}

fn save_file(
    trigger: &Stream<()>,
    filename: &Signal<Option<PathBuf>>,
    win: &gtk::Window,
) -> Stream<Option<PathBuf>> {
    let win_ = win.wrap_fragile();
    filename
        .snapshot(trigger, |a, _| a)
        // obtain the filename from a dialog
        .map_n(move |optname, sender| {
            if let Some(name) = optname.into_owned() {
                sender.send(name);
            } else {
                let dialog = gtk::FileChooserDialog::with_buttons(
                    Some("Save file as"),
                    Some(win_.get()),
                    gtk::FileChooserAction::Save,
                    &[
                        ("_Cancel", gtk::ResponseType::Cancel),
                        ("_Save", gtk::ResponseType::Accept),
                    ],
                );
                dialog.show();
                dialog.connect_response(move |dlg, resp| {
                    let fname = dlg.filename();
                    dlg.hide();
                    if resp == gtk::ResponseType::Accept {
                        println!("got filename {:?}", fname);
                        if let Some(name) = fname {
                            sender.send(name);
                        }
                    }
                });
            }
        })
        // save the file
        .map(|name| {
            println!("saving file: {:?}", name);
            Some(name.into_owned())
        })
}
