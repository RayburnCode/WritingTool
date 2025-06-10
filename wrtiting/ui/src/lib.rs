//! This crate contains all shared UI for the workspace.

pub mod auth;
mod hero;
pub use hero::Hero;

mod echo;
pub use echo::Echo;

pub mod editor_sidebar;
pub use editor_sidebar::{DocumentNode, NodeType, TreeNode, TreeView, Sidebar};

pub mod landing_page;
pub use landing_page::LandingPage;


// mod.rs
pub mod button;
pub mod input;
pub mod avatar;
pub mod avatar_drop;
pub mod navbar_drop;
pub mod table;
pub mod loading_spinner;
pub mod search;
pub mod sidebar;

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

pub use input::{Input, InputSize, InputType, InputProps, TextInput, PasswordInput, DateInput, NumberInput,SelectInput};

pub use navbar_drop::{NavDrop, 
    NavMenuItem, 
};
pub use search::Search;
pub use sidebar::{TraditionalSidebar, SidebarItem,};