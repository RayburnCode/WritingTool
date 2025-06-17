use dioxus::prelude::*;
use crate::views::profile::{
    PersonalInfoSection, ProfileHeader, 
    FormActions, SecuritySection, NotificationsSection, AppearanceSection
};

#[component]
pub fn Profile() -> Element {
    // Profile data (matches profiles table)
    let  first_name = use_signal(|| "John".to_string());
    let  last_name = use_signal(|| "Doe".to_string());
    let  bio = use_signal(|| "Software developer".to_string());
    let  website_url = use_signal(|| "https://example.com".to_string());
    let  avatar_url = use_signal(|| Some("https://example.com/avatar.jpg".to_string()));
    let  is_editing = use_signal(|| false);
    let  is_uploading = use_signal(|| false);

    // User preferences (matches user_preferences table)
    let  app_theme = use_signal(|| "light".to_string());
    let  email_notifications = use_signal(|| true);
    let  sms_notifications = use_signal(|| false);
    let  enable_2fa = use_signal(|| false);



    rsx! {
        div { class: "space-y-8",
            ProfileHeader {
                avatar_url,
                is_editing,
                is_uploading,
                first_name: first_name.clone(),
                last_name: last_name.clone(),
            }
            // Personal Information Section
            div { class: "max-w-4xl mx-auto p-6 bg-white rounded-lg shadow",
                PersonalInfoSection {
                    first_name,
                    last_name,
                    bio,
                    website_url,
                    is_editing,
                }

                FormActions { is_editing }
            }

            // Account Settings Section
            div { class: "max-w-4xl mx-auto p-6 space-y-6",
                div { class: "mb-8",
                    h1 { class: "text-2xl font-bold text-gray-900", "Account Settings" }
                    p { class: "text-gray-600", "Manage your security and application preferences" }
                }
                SecuritySection { enable_2fa }
                NotificationsSection { email_notifications, sms_notifications }
                AppearanceSection { app_theme }
            }
        }
    }
}