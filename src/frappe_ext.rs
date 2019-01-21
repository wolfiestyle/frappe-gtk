use crate::types::Fragile;
use frappe::{Signal, Stream};
use gtk::prelude::*;

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

/// Extension trait for frappe signals.
pub trait SignalExt<T> {
    /// Wraps the Signal values in Fragile.
    fn wrap_fragile(&self) -> Signal<Fragile<T>>;
}

impl<T> SignalExt<T> for Signal<T>
where
    T: Clone + 'static,
{
    fn wrap_fragile(&self) -> Signal<Fragile<T>> {
        self.map(Fragile::new)
    }
}

/// Extension trait for `Signal<Option<T>>`.
pub trait SignalOptExt<T> {
    /// Wraps the Signal's `Some` values in Fragile.
    fn wrap_opt_fragile(&self) -> Signal<Option<Fragile<T>>>;
}

impl<T> SignalOptExt<T> for Signal<Option<T>>
where
    T: Clone + 'static,
{
    fn wrap_opt_fragile(&self) -> Signal<Option<Fragile<T>>> {
        self.map(|opt| opt.map(Fragile::new))
    }
}
