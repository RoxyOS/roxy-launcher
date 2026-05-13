use crate::ui::toolbar::ToolBar;

pub mod dialogs;
pub mod instance_grid;
pub mod side_bar;
pub mod toolbar;

#[derive(Default, Debug)]
pub struct RoxyMainComponents {
    pub toolbar: ToolBar,
}
