use dioxus::prelude::*;

use views::{AppLayout, Blog, Home, Editor, FocusMode, Settings, HelpMain, HelpFaq, HelpContact, Login, NotFound};

mod views;

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
    NotFound {},
}





// Update the path if your CSS file is located elsewhere, e.g. "/assets/css/tailwind.css"
const MAIN_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    rsx! {

        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}


