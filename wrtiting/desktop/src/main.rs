use dioxus::prelude::*;

use state::Theme;
use views::{AppLayout, Blog, Home, Editor, FocusMode, Settings, HelpMain, HelpFaq, HelpContact, Login, NotFound};
mod views;
mod state;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
        
        #[route("/blog/:id")]
        Blog { id: i32 },
        
        
        #[route("/editor")]
        Editor {},
        
        #[route("/focus")]
        FocusMode {},
        
        #[route("/settings")]
        Settings {},
        
        #[nest("/help")]
            #[route("/")]
            HelpMain {},
            
            #[route("/faq")]
            HelpFaq {},
            
            #[route("/contact")]
            HelpContact {},
        #[end_nest]
        
    #[end_layout]
    
    #[route("/login")]
    Login {},
    
    #[route("/404")]
    NotFound { route: Vec<String> },
}





// Update the path if your CSS file is located elsewhere, e.g. "/assets/css/tailwind.css"
const MAIN_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize theme and provide it to the tree
    let theme = use_signal(|| Theme::Light);
    provide_context(theme);

    // Optional: Apply theme-dependent CSS globally
    let bg_color = match *theme.read() {
        Theme::Dark => "bg-gray-900",
        Theme::Light => "bg-white",
    };
    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}


