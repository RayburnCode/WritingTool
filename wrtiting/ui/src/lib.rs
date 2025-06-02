//! This crate contains all shared UI for the workspace.

mod auth;

mod hero;
pub use hero::Hero;

mod echo;
pub use echo::Echo;

pub mod sidebar;
pub use sidebar::{DocumentNode, NodeType, TreeNode, TreeView};

pub mod landing_page;
pub use landing_page::LandingPage;


// mod.rs
pub mod button;
pub mod avatar;
pub mod avatar_drop;
pub mod table;
pub mod loading_spinner;


// Re-export from modules
pub use button::{Button, ButtonSize, ButtonScheme, ButtonType};
pub use avatar::Avatar;
pub use avatar_drop::{AvatarDrop, 
    MenuItem, 
};
pub use table::{Table, TableProps, TableHead, TableHeadProps, TableBody, TableBodyProps, TableRow, TableRowProps, TableCell, TableCellProps, TableHeaderCell, TableHeaderCellProps, TableFoot, TableFootProps};
pub use loading_spinner::{
    SpinnerSize,
    SpinnerColor,
    SpinnerProps
};
