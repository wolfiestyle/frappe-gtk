use crate::types::Fragile;
use frappe::Stream;
use gtk::prelude::*;
use gtk::{ResponseType, Widget};

/// Extension trait for frappe streams.
pub trait StreamExt<T> {
    /// Executes the rest of this stream chain on the main thread.
    fn to_main_thread(&self) -> Stream<T>
    where
        T: Clone + Send + 'static;
    /// Wraps each Stream value in a Fragile.
    fn wrap_fragile(&self) -> Stream<Fragile<T>>
    where
        T: Clone + 'static;
    /// Shows a dialog and maps it's response values.
    fn map_dialog<F, R>(&self, f: F) -> Stream<R>
    where
        F: Fn(&T, ResponseType) -> R + Clone + Send + Sync + 'static,
        T: IsA<Widget> + DialogExt + 'static,
        R: 'static;
    /// Shows a dialog and returns it's response value.
    ///
    /// This hides the dialog after receiving a response.
    /// If you don't want that, use `map_dialog` instead.
    fn run_dialog(&self) -> Stream<ResponseType>
    where
        T: IsA<Widget> + DialogExt + 'static,
    {
        self.map_dialog(|dlg, resp| {
            dlg.hide();
            resp
        })
    }
}

impl<T> StreamExt<T> for Stream<T> {
    fn to_main_thread(&self) -> Stream<T>
    where
        T: Clone + Send + 'static,
    {
        self.map_n(|val, sender| {
            let mut val = Some(val.into_owned());
            glib::idle_add(move || {
                if let Some(val) = val.take() {
                    sender.send(val);
                }
                Continue(false)
            });
        })
    }

    fn wrap_fragile(&self) -> Stream<Fragile<T>>
    where
        T: Clone + 'static,
    {
        self.map(|val| Fragile::new(val.into_owned()))
    }

    fn map_dialog<F, R>(&self, f: F) -> Stream<R>
    where
        F: Fn(&T, ResponseType) -> R + Clone + Send + Sync + 'static,
        T: DialogExt + WidgetExt + 'static,
        R: 'static,
    {
        self.map_n(move |dialog, sender| {
            dialog.show();
            let f = f.clone();
            dialog.connect_response(move |dlg, resp| sender.send(f(dlg, resp)));
        })
    }
}

pub trait StreamOptExt<T> {
    fn map_opt_dialog<F, R>(&self, f: F) -> Stream<Option<R>>
    where
        F: Fn(&T, ResponseType) -> R + Clone + Send + Sync + 'static,
        T: DialogExt + WidgetExt + 'static,
        R: 'static;

    fn run_opt_dialog(&self) -> Stream<Option<ResponseType>>
    where
        T: Clone + IsA<Widget> + DialogExt + 'static,
    {
        self.map_opt_dialog(|dlg, resp| {
            dlg.hide();
            resp
        })
    }
}

impl<T> StreamOptExt<T> for Stream<Option<T>> {
    fn map_opt_dialog<F, R>(&self, f: F) -> Stream<Option<R>>
    where
        F: Fn(&T, ResponseType) -> R + Clone + Send + Sync + 'static,
        T: DialogExt + WidgetExt + 'static,
        R: 'static,
    {
        self.map_n(move |optdlg, sender| {
            if let Some(dialog) = optdlg.as_ref() {
                dialog.show();
                let f = f.clone();
                dialog.connect_response(move |dlg, resp| sender.send(Some(f(dlg, resp))));
            } else {
                sender.send(None)
            }
        })
    }
}

/// Extension trait for `Stream<Fragile<T>>`.
pub trait StreamFragileExt<T> {
    /// Extracts the inner value from Fragile objects.
    fn unwrap_fragile(&self) -> Stream<T>
    where
        T: Clone + 'static;
}

impl<T> StreamFragileExt<T> for Stream<Fragile<T>> {
    fn unwrap_fragile(&self) -> Stream<T>
    where
        T: Clone + 'static,
    {
        self.map(|f| f.into_owned().into_inner())
    }
}
