use crate::types::*;
use frappe::Stream;
use gtk::{Inhibit, IsA, Widget};

/// Extension trait for `gtk::BoxExt`.
pub trait FrpBoxExt {
    /// Appends widgets received from a Stream.
    fn stream_pack_start<W: IsA<Widget> + 'static>(&self, stream: &Stream<PackArgs<W>>);
    /// Prepends widgets received from a Stream.
    fn stream_pack_end<W: IsA<Widget> + 'static>(&self, stream: &Stream<PackArgs<W>>);
}

impl<T> FrpBoxExt for T
where
    T: gtk::BoxExt + gtk::ObjectExt + 'static,
{
    fn stream_pack_start<W: IsA<Widget> + 'static>(&self, stream: &Stream<PackArgs<W>>) {
        gtk_observe!(stream, |args| self.pack_start(
            &args.child,
            args.expand,
            args.fill,
            args.padding
        ))
    }

    fn stream_pack_end<W: IsA<Widget> + 'static>(&self, stream: &Stream<PackArgs<W>>) {
        gtk_observe!(stream, |args| self.pack_end(
            &args.child,
            args.expand,
            args.fill,
            args.padding
        ))
    }
}

/// Extension trait for `gtk::ButtonExt`.
pub trait FrpButtonExt {
    /// Sets the label with the values from a Stream.
    fn stream_label(&self, stream: &Stream<String>);
    /// Returns a `clicked` event Stream.
    fn clicked_events(&self) -> Stream<()>;
}

impl<T> FrpButtonExt for T
where
    T: gtk::ButtonExt + gtk::ObjectExt + 'static,
{
    fn stream_label(&self, stream: &Stream<String>) {
        gtk_observe!(stream, |s| self.set_label(&s))
    }

    fn clicked_events(&self) -> Stream<()> {
        connect_stream!(self.connect_clicked)
    }
}

/// Extension trait for gtk CheckMenuItem
trait FrpCheckMenuItemExt {
    fn toggled_events(&self) -> Stream<ToggleState>;
}

impl<T> FrpCheckMenuItemExt for T
where
    T: gtk::CheckMenuItemExt,
{
    fn toggled_events(&self) -> Stream<ToggleState> {
        connect_stream!(self.connect_toggled, |this| ToggleState {
            active: this.get_active(),
            inconsistent: this.get_inconsistent(),
        })
    }
}

/// Extension trait for `gtk::ContainerExt`.
pub trait FrpContainerExt {
    /// Adds the widgets received from a Stream.
    fn stream_add<W: IsA<Widget> + 'static>(&self, stream: &Stream<W>);
    /// Removes the widgets received from a Stream.
    fn stream_remove<W: IsA<Widget> + 'static>(&self, stream: &Stream<W>);
    /// Returns an `add` event Stream.
    fn add_events(&self) -> Stream<Widget>;
    /// Returns a `remove` event Stream.
    fn remove_events(&self) -> Stream<Widget>;
}

impl<T> FrpContainerExt for T
where
    T: gtk::ContainerExt + gtk::ObjectExt + 'static,
{
    fn stream_add<W: IsA<Widget> + 'static>(&self, stream: &Stream<W>) {
        gtk_observe!(stream, |widget| self.add(widget.as_ref()))
    }

    fn stream_remove<W: IsA<Widget> + 'static>(&self, stream: &Stream<W>) {
        gtk_observe!(stream, |widget| self.remove(widget.as_ref()))
    }

    fn add_events(&self) -> Stream<Widget> {
        connect_stream!(self.connect_add, |_, widget| widget)
    }

    fn remove_events(&self) -> Stream<Widget> {
        connect_stream!(self.connect_remove, |_, widget| widget)
    }
}

/// Extension trait for `gtk::DialogExt`.
pub trait FrpDialogExt {
    /// Returns a `close` event Stream.
    fn close_events(&self) -> Stream<()>;
    /// Returns a `response` event Stream.
    fn response_events(&self) -> Stream<i32>;
}

impl<T> FrpDialogExt for T
where
    T: gtk::DialogExt,
{
    fn close_events(&self) -> Stream<()> {
        connect_stream!(self.connect_close)
    }

    fn response_events(&self) -> Stream<i32> {
        connect_stream!(self.connect_response, |_, id| id)
    }
}

/// Extension trait for `gtk::EntryExt`
pub trait FrpEntryExt {
    fn stream_text(&self, stream: &Stream<String>);
    fn activate_events(&self) -> Stream<()>;
    fn backspace_events(&self) -> Stream<()>;
}

impl<T> FrpEntryExt for T
where
    T: gtk::EntryExt + gtk::ObjectExt + 'static,
{
    fn stream_text(&self, stream: &Stream<String>) {
        gtk_observe!(stream, |s| self.set_text(&s))
    }

    fn activate_events(&self) -> Stream<()> {
        connect_stream!(self.connect_activate)
    }

    fn backspace_events(&self) -> Stream<()> {
        connect_stream!(self.connect_backspace)
    }
}

/// Extension trait for `gtk::ExpanderExt`.
pub trait FrpExpanderExt {
    fn stream_expanded(&self, stream: &Stream<bool>);
}

impl<T> FrpExpanderExt for T
where
    T: gtk::ExpanderExt + gtk::ObjectExt + 'static,
{
    /// Sets the expanded property using the values from a Stream.
    fn stream_expanded(&self, stream: &Stream<bool>) {
        gtk_observe!(stream, |b| self.set_expanded(*b))
    }
}

/// Extension trait for `gtk::GridExt`.
pub trait FrpGridExt {
    fn stream_attach<W: IsA<Widget> + 'static>(&self, stream: &Stream<GridArgs<W>>);
}

impl<T> FrpGridExt for T
where
    T: gtk::GridExt + gtk::ObjectExt + 'static,
{
    fn stream_attach<W: IsA<Widget> + 'static>(&self, stream: &Stream<GridArgs<W>>) {
        gtk_observe!(stream, |args| self.attach(
            &args.child,
            args.left,
            args.top,
            args.width,
            args.height
        ))
    }
}

/// Extension trait for `gtk::GtkWindowExt`.
pub trait FrpGtkWindowExt {
    fn stream_position(&self, stream: &Stream<gtk::WindowPosition>);
    fn stream_size(&self, stream: &Stream<(i32, i32)>);
    fn stream_title(&self, stream: &Stream<String>);
    fn activate_default_events(&self) -> Stream<()>;
    fn activate_focus_events(&self) -> Stream<()>;
}

impl<T> FrpGtkWindowExt for T
where
    T: gtk::GtkWindowExt + gtk::ObjectExt + 'static,
{
    fn stream_position(&self, stream: &Stream<gtk::WindowPosition>) {
        gtk_observe!(stream, |pos| self.set_position(*pos))
    }

    fn stream_size(&self, stream: &Stream<(i32, i32)>) {
        gtk_observe!(stream, |args| self.resize(args.0, args.1))
    }

    fn stream_title(&self, stream: &Stream<String>) {
        gtk_observe!(stream, |s| self.set_title(&s))
    }

    fn activate_default_events(&self) -> Stream<()> {
        connect_stream!(self.connect_activate_default)
    }

    fn activate_focus_events(&self) -> Stream<()> {
        connect_stream!(self.connect_activate_focus)
    }
}

/// Extension trait for `gtk::LabelExt`.
pub trait FrpLabelExt {
    fn stream_label(&self, stream: &Stream<String>);
}

impl<T> FrpLabelExt for T
where
    T: gtk::LabelExt + gtk::ObjectExt + 'static,
{
    fn stream_label(&self, stream: &Stream<String>) {
        gtk_observe!(stream, |s| self.set_label(&s))
    }
}

/// Extension trait for `gtk::LinkButtonExt`.
pub trait FrpLinkButtonExt {
    fn activate_link_events(&self, inhibit: bool) -> Stream<()>;
}

impl<T> FrpLinkButtonExt for T
where
    T: gtk::LinkButtonExt,
{
    fn activate_link_events(&self, inhibit: bool) -> Stream<()> {
        connect_stream!(self.connect_activate_link, |_| (); Inhibit(inhibit))
    }
}

/// Extension trait for `gtk::MenuItemExt`.
pub trait FrpMenuItemExt {
    fn activate_events(&self) -> Stream<()>;
}

impl<T> FrpMenuItemExt for T
where
    T: gtk::MenuItemExt,
{
    fn activate_events(&self) -> Stream<()> {
        connect_stream!(self.connect_activate)
    }
}

/// Extension trait for `gtk::NotebookExt`.
pub trait FrpNotebookExt {
    fn page_added_events(&self) -> Stream<NotebookPage>;
    fn page_removed_events(&self) -> Stream<NotebookPage>;
    fn page_reordered_events(&self) -> Stream<NotebookPage>;
    fn switch_page_events(&self) -> Stream<NotebookPage>;
}

impl<T> FrpNotebookExt for T
where
    T: gtk::NotebookExt,
{
    fn page_added_events(&self) -> Stream<NotebookPage> {
        connect_stream!(self.connect_page_added, |_, w, page_num| NotebookPage {
            child: w.clone(),
            page_num,
        })
    }

    fn page_removed_events(&self) -> Stream<NotebookPage> {
        connect_stream!(self.connect_page_removed, |_, w, page_num| NotebookPage {
            child: w.clone(),
            page_num,
        })
    }

    fn page_reordered_events(&self) -> Stream<NotebookPage> {
        connect_stream!(self.connect_page_reordered, |_, w, page_num| NotebookPage {
            child: w.clone(),
            page_num,
        })
    }

    fn switch_page_events(&self) -> Stream<NotebookPage> {
        connect_stream!(self.connect_switch_page, |_, w, page_num| NotebookPage {
            child: w.clone(),
            page_num,
        })
    }
}

/// Extension trait for `gtk::OverlayExt`.
pub trait FrpOverlayExt {
    fn stream_add_overlay<W: IsA<Widget> + 'static>(&self, stream: &Stream<W>);
}

impl<T> FrpOverlayExt for T
where
    T: gtk::OverlayExt + gtk::ObjectExt + 'static,
{
    fn stream_add_overlay<W: IsA<Widget> + 'static>(&self, stream: &Stream<W>) {
        gtk_observe!(stream, |widget| self.add_overlay(widget.as_ref()))
    }
}

/// Extension trait for `gtk::RangeExt`.
pub trait FrpRangeExt {
    fn change_value_events(&self, inhibit: bool) -> Stream<RangeValue>;
}

impl<T> FrpRangeExt for T
where
    T: gtk::RangeExt,
{
    fn change_value_events(&self, inhibit: bool) -> Stream<RangeValue> {
        connect_stream!(self.connect_change_value, |_, scroll, value| RangeValue{ scroll, value }; Inhibit(inhibit))
    }
}

/// Extension trait for `gtk::SpinnerExt`.
pub trait FrpSpinnerExt {
    fn stream_active(&self, stream: &Stream<bool>);
}

impl<T> FrpSpinnerExt for T
where
    T: gtk::SpinnerExt + gtk::ObjectExt + 'static,
{
    fn stream_active(&self, stream: &Stream<bool>) {
        gtk_observe!(stream, |active| self.set_property_active(*active))
    }
}

/// Extension trait for `gtk::TextBufferExt`.
pub trait FrpTextBufferExt {
    fn stream_modified(&self, stream: &Stream<bool>);
    fn stream_text(&self, stream: &Stream<String>);
    fn changed_events(&self) -> Stream<()>;
    fn modified_changed_events(&self) -> Stream<bool>;
}

impl<T> FrpTextBufferExt for T
where
    T: gtk::TextBufferExt + gtk::ObjectExt + 'static,
{
    fn stream_modified(&self, stream: &Stream<bool>) {
        gtk_observe!(stream, |b| self.set_modified(*b))
    }

    fn stream_text(&self, stream: &Stream<String>) {
        gtk_observe!(stream, |s| self.set_text(&s))
    }

    fn changed_events(&self) -> Stream<()> {
        connect_stream!(self.connect_changed)
    }

    fn modified_changed_events(&self) -> Stream<bool> {
        connect_stream!(self.connect_modified_changed, |this| this.get_modified())
    }
}

/// Extension trait for `gtk::ToggleButtonExt`.
trait FrpToggleButtonExt {
    fn toggled_events(&self) -> Stream<ToggleState>;
}

impl<T> FrpToggleButtonExt for T
where
    T: gtk::ToggleButtonExt,
{
    fn toggled_events(&self) -> Stream<ToggleState> {
        connect_stream!(self.connect_toggled, |this| ToggleState {
            active: this.get_active(),
            inconsistent: this.get_inconsistent(),
        })
    }
}

/// Extension trait for `gtk::ButtonExt`.
pub trait FrpToolButtonExt {
    /// Returns a `clicked` event Stream.
    fn clicked_events(&self) -> Stream<()>;
}

impl<T> FrpToolButtonExt for T
where
    T: gtk::ToolButtonExt,
{
    fn clicked_events(&self) -> Stream<()> {
        connect_stream!(self.connect_clicked)
    }
}

/// Extension trait for `gtk::WidgetExt`.
pub trait FrpWidgetExt {
    fn stream_sensitive(&self, stream: &Stream<bool>);
    fn stream_visible(&self, stream: &Stream<bool>);
    fn delete_events(&self, inhibit: bool) -> Stream<()>;
    fn enter_notify_events(&self, inhibit: bool) -> Stream<gdk::EventCrossing>;
    fn leave_notify_events(&self, inhibit: bool) -> Stream<gdk::EventCrossing>;
    fn motion_notify_events(&self, inhibit: bool) -> Stream<gdk::EventMotion>;
    fn show_events(&self) -> Stream<()>;
    fn hide_events(&self) -> Stream<()>;
    fn show_(self) -> Self;
    fn show_all_(self) -> Self;
}

impl<T> FrpWidgetExt for T
where
    T: gtk::WidgetExt + gtk::ObjectExt + 'static,
{
    fn stream_sensitive(&self, stream: &Stream<bool>) {
        gtk_observe!(stream, |b| self.set_sensitive(*b))
    }

    fn stream_visible(&self, stream: &Stream<bool>) {
        gtk_observe!(stream, |b| self.set_visible(*b))
    }

    fn delete_events(&self, inhibit: bool) -> Stream<()> {
        connect_stream!(self.connect_delete_event, |_, _| (); Inhibit(inhibit))
    }

    fn enter_notify_events(&self, inhibit: bool) -> Stream<gdk::EventCrossing> {
        connect_stream!(self.connect_enter_notify_event, |_, ev| ev; Inhibit(inhibit))
    }

    fn leave_notify_events(&self, inhibit: bool) -> Stream<gdk::EventCrossing> {
        connect_stream!(self.connect_leave_notify_event, |_, ev| ev; Inhibit(inhibit))
    }

    fn motion_notify_events(&self, inhibit: bool) -> Stream<gdk::EventMotion> {
        connect_stream!(self.connect_motion_notify_event, |_, ev| ev; Inhibit(inhibit))
    }

    fn show_events(&self) -> Stream<()> {
        connect_stream!(self.connect_show)
    }

    fn hide_events(&self) -> Stream<()> {
        connect_stream!(self.connect_hide)
    }

    fn show_(self) -> Self {
        self.show();
        self
    }

    fn show_all_(self) -> Self {
        self.show_all();
        self
    }
}

/// Extension trait for `gtk::ObjectExt`.
pub trait FrpObjectExt {
    fn wrap_fragile(&self) -> Fragile<Self>
    where
        Self: Sized;
}

impl<T> FrpObjectExt for T
where
    T: gtk::ObjectExt + Clone,
{
    fn wrap_fragile(&self) -> Fragile<Self> {
        Fragile::new(self.clone())
    }
}
