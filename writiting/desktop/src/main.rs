use dioxus::prelude::*;

use views::{AppLayout, Blog, Home, Editor, FocusMode, NotFound};
use views::profile::{AccountSettings, Profile};
use views::legal::{PrivacyPolicy, TermsOfService};
use views::admin::{AdminDashboard, AdminUsers, AdminReports, AdminSettings};
use views::ai::AIChat;
use views::help::{HelpMain};

use ui::auth::{Login, Register, ResetPassword};
mod views;
mod state;
use std::env;
use std::error::Error;



#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
        
        #[route("/blog/:initial_id")]
        Blog { initial_id: i32 },
        
        #[route("/editor")]
        Editor {},

        #[route("/focus")]
        FocusMode {},
        
        #[route("/settings")]
        AccountSettings {},

        #[route("/profile")]
        Profile {},
        #[route("/help")]
        HelpMain {},
        
         #[nest("/auth")]

            #[route("/login")]
            Login {},
            
            #[route("/register")]
            Register {},
            
            #[route("/reset-password")]
            ResetPassword {},
        #[end_nest]
    
        // #[nest("/help")]
        //     #[route("/")]
        //     HelpMain {},
            
        //     #[route("/faq")]
        //     HelpFaq {},
            
        //     #[route("/contact")]
        //     HelpContact {},
        // #[end_nest]

        #[nest("/admin")]
             #[route("/")]
             AdminDashboard {},
   
            #[route("/users")]
            AdminUsers {},
   
            #[route("/reports")]
            AdminReports {},

            #[route("/settings")]
            AdminSettings {},
        #[end_nest]

        #[nest("/legal")]
            #[route("/privacy")]
             PrivacyPolicy {},
   
            #[route("/terms")]
            TermsOfService {},
        #[end_nest]

        #[nest("/ai")]
            #[route("/")]
             AIChat {},
        #[end_nest]
        
    #[end_layout]

    #[route("/404")]
    NotFound { route: Vec<String> },
}





// Update the path if your CSS file is located elsewhere, e.g. "/assets/css/tailwind.css"
const MAIN_CSS: Asset = asset!("/assets/tailwind.css");

fn main() -> Result<(), Box<dyn Error>> {


        // Load environment variables from .env file.
    // Fails if .env file not found, not readable or invalid.
    dotenvy::dotenv()?;

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    Ok(dioxus::launch(App))
}

    


#[component]
fn App() -> Element {
    
    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }

}



