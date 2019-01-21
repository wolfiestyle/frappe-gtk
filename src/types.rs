pub use fragile::Fragile;

/// Box packing arguments.
#[derive(Debug, Clone)]
pub struct PackArgs<W> {
    pub child: W,
    pub expand: bool,
    pub fill: bool,
    pub padding: u32,
}

/// State of a toggleable item.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ToggleState {
    pub active: bool,
    pub inconsistent: bool,
}

/// Widget and position arguments for `gtk::Grid`.
#[derive(Debug, Clone)]
pub struct GridArgs<W> {
    pub child: W,
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

/// Notebook child widget and page number.
#[derive(Debug, Clone)]
pub struct NotebookPage {
    pub child: gtk::Widget,
    pub page_num: u32,
}

/// Value of a `gtk::Range`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeValue {
    pub scroll: gtk::ScrollType,
    pub value: f64,
}
