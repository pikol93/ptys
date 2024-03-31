#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub enum DisplayedView {
    #[default]
    Menu,
    Connections,
}

#[derive(Default)]
pub struct ApplicationModel {
    pub displayed_view: DisplayedView,
}
