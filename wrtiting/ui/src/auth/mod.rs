use dioxus::prelude::*;

pub mod login;
pub mod register;
pub mod reset_password;
pub mod state;

pub use login::Login;
pub use register::Register;
pub use reset_password::ResetPassword;
pub use state::{Notification, NotificationType, LoginForm, FormErrors, LoginState, LoginResult, simulate_login_api};



#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum AuthRoute {
        #[route("/login")]
        Login {},
        #[route("/register")]
        Register {},
        #[route("/reset-password")]
        ResetPassword {},



}
