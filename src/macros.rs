/// Lifts the specified Gtk method into a Signal.
#[macro_export]
macro_rules! gtk_lift {
    ($obj:ident . $method:ident) => {{
        let this = $crate::types::Fragile::new($obj.clone());
        frappe::Signal::from_fn(move || this.get().$method())
    }};
}

/// Calls the specified Gtk method using the values from a Stream.
#[macro_export]
macro_rules! gtk_observe {
    ($stream:expr , | $($args:pat),+ | $obj:ident . $method:ident ( $($e:expr),+ )) => ({
        let weak = $crate::types::Fragile::new($obj.downgrade());
        $stream.observe_strong(move |$($args),+| {
            weak.get().upgrade().map(|obj| obj.$method($($e),+));
        });
    });
}

/// Sends the specified object's Gtk events into a Stream.
#[macro_export]
macro_rules! connect_stream {
    ($obj:ident . $method:ident) => ({
        $crate::connect_stream!($obj.$method, |_| ())
    });

    ($obj:ident . $method:ident , | $($args:pat),+ | $e:expr $(;$ret:expr)?) => ({
        let sink = frappe::Sink::new();
        let stream = sink.stream();
        $obj.$method(move |$($args),+| { sink.send($e) $(;$ret)? });
        stream
    });
}
