use dioxus::prelude::*;
use dioxus_router::prelude::*;

// Form data structure
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct RegisterForm {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub accept_terms: bool,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct RegisterErrors {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub confirm_password: Option<String>,
    pub accept_terms: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct RegisterState {
    pub data: RegisterForm,
    pub errors: RegisterErrors,
    pub loading: bool,
}

#[component]
pub fn RegisterPage(cx: Scope) -> Element {
    let form_state = use_state(cx, RegisterState::default);
    let navigator = use_navigator(cx);

    // Handle form submission
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        
        form_state.with_mut(|state| state.loading = true);
        
        // Validate passwords match
        if form_state.get().data.password != form_state.get().data.confirm_password {
            form_state.with_mut(|state| {
                state.errors.password = Some("Passwords do not match".to_string());
                state.errors.confirm_password = Some("Passwords do not match".to_string());
                state.loading = false;
            });
            return;
        }
        
        // Validate terms accepted
        if !form_state.get().data.accept_terms {
            form_state.with_mut(|state| {
                state.errors.accept_terms = Some("You must accept the terms and conditions".to_string());
                state.loading = false;
            });
            return;
        }
        
        // In a real app, you would make an async call to your backend here
        async {
            // Simulate API call delay
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            
            // This would be your actual API response handling
            let result = simulate_register_api(&form_state.get().data).await;
            
            match result {
                RegisterResult::Success => {
                    navigator.push("/login?registered=true");
                }
                RegisterResult::Error(errors) => {
                    form_state.with_mut(|state| {
                        state.errors = errors;
                        state.loading = false;
                    });
                }
            }
        }
    };

    // Handle input changes
    let handle_input_change = move |field: &str, value: String| {
        form_state.with_mut(|state| {
            // Clear any existing errors when user types
            match field {
                "name" => {
                    state.data.name = value;
                    state.errors.name = None;
                }
                "email" => {
                    state.data.email = value;
                    state.errors.email = None;
                }
                "password" => {
                    state.data.password = value;
                    state.errors.password = None;
                    // Also clear confirm password error if passwords now match
                    if state.data.password == state.data.confirm_password {
                        state.errors.confirm_password = None;
                    }
                }
                "confirm_password" => {
                    state.data.confirm_password = value;
                    state.errors.confirm_password = None;
                    // Also clear password error if passwords now match
                    if state.data.password == state.data.confirm_password {
                        state.errors.password = None;
                    }
                }
                _ => (),
            }
        });
    };

    // Handle terms checkbox
    let handle_terms_change = move |checked: bool| {
        form_state.with_mut(|state| {
            state.data.accept_terms = checked;
            state.errors.accept_terms = None;
        });
    };

    cx.render(rsx! {
        div { class: "bg-gray-50 antialiased dark:bg-gray-900 min-h-screen",
            main { class: "w-full bg-gray-50 dark:bg-gray-900",
                div { class: "pt:mt-0 mx-auto flex flex-col items-center justify-center px-6 pt-8 md:h-screen dark:bg-gray-900",
                    Link { 
                        to: "/", 
                        class: "mb-8 flex items-center justify-center text-2xl font-semibold lg:mb-10 dark:text-white",
                        img { class: "mr-4 h-11", src: "/static/logo.svg", alt: "Logo" }
                        span { "Rayburn LP" }
                    }
                    
                    div { class: "flex w-full max-w-xl flex-col divide-gray-200 rounded-lg border-gray-200 bg-white p-4 text-gray-500 shadow-md sm:p-6 dark:divide-gray-700 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400",
                        h1 { class: "text-2xl font-bold text-gray-900 dark:text-white", "Create an Account" }
                        
                        form { 
                            action: "/register", 
                            method: "POST", 
                            class: "mt-8 space-y-6",
                            onsubmit: handle_submit,
                            
                            // Name field
                            div {
                                label { 
                                    class: "block space-y-2 text-sm font-medium text-gray-900 rtl:text-right dark:text-white",
                                    span { "Name" }
                                    div { class: "relative w-full",
                                        input {
                                            id: "name",
                                            name: "name",
                                            placeholder: "Your name",
                                            required: true,
                                            value: "{form_state.get().data.name}",
                                            oninput: move |e| handle_input_change("name", e.value.clone()),
                                            class: "w-full rounded border border-gray-300 bg-gray-50 p-2.5 text-gray-900 focus:border-primary-600 focus:ring-primary-600 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-primary-500 dark:focus:ring-primary-500 sm:text-sm"
                                        }
                                        if let Some(err) = &form_state.get().errors.name {
                                            p { class: "mt-2 text-sm text-red-600 dark:text-red-500", "{err}" }
                                        }
                                    }
                                }
                            }
                            
                            // Email field
                            div {
                                label { 
                                    class: "block space-y-2 text-sm font-medium text-gray-900 rtl:text-right dark:text-white",
                                    span { "Your email" }
                                    div { class: "relative w-full",
                                        input {
                                            id: "email",
                                            name: "email",
                                            placeholder: "name@company.com",
                                            required: true,
                                            r#type: "email",
                                            value: "{form_state.get().data.email}",
                                            oninput: move |e| handle_input_change("email", e.value.clone()),
                                            class: "w-full rounded border border-gray-300 bg-gray-50 p-2.5 text-gray-900 focus:border-primary-600 focus:ring-primary-600 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-primary-500 dark:focus:ring-primary-500 sm:text-sm"
                                        }
                                        div { class: "absolute inset-y-0 end-0 flex items-center pe-2.5 text-gray-500 dark:text-gray-400" }
                                        if let Some(err) = &form_state.get().errors.email {
                                            p { class: "mt-2 text-sm text-red-600 dark:text-red-500", "{err}" }
                                        }
                                    }
                                }
                            }
                            
                            // Password field
                            div {
                                label { 
                                    class: "block space-y-2 text-sm font-medium text-gray-900 rtl:text-right dark:text-white",
                                    span { "Your password" }
                                    div { class: "relative w-full",
                                        input {
                                            id: "password",
                                            name: "password",
                                            placeholder: "password",
                                            required: true,
                                            r#type: "password",
                                            value: "{form_state.get().data.password}",
                                            oninput: move |e| handle_input_change("password", e.value.clone()),
                                            class: "w-full rounded border border-gray-300 bg-gray-50 p-2.5 text-gray-900 focus:border-primary-600 focus:ring-primary-600 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-primary-500 dark:focus:ring-primary-500 sm:text-sm"
                                        }
                                        div { class: "absolute inset-y-0 end-0 flex items-center pe-2.5 text-gray-500 dark:text-gray-400" }
                                        if let Some(err) = &form_state.get().errors.password {
                                            p { class: "mt-2 text-sm text-red-600 dark:text-red-500", "{err}" }
                                        }
                                    }
                                }
                            }
                            
                            // Confirm Password field
                            div {
                                label { 
                                    class: "block space-y-2 text-sm font-medium text-gray-900 rtl:text-right dark:text-white",
                                    span { "Confirm password" }
                                    div { class: "relative w-full",
                                        input {
                                            id: "confirm-password",
                                            name: "confirm-password",
                                            placeholder: "password",
                                            required: true,
                                            r#type: "password",
                                            value: "{form_state.get().data.confirm_password}",
                                            oninput: move |e| handle_input_change("confirm_password", e.value.clone()),
                                            class: "w-full rounded border border-gray-300 bg-gray-50 p-2.5 text-gray-900 focus:border-primary-600 focus:ring-primary-600 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-primary-500 dark:focus:ring-primary-500 sm:text-sm"
                                        }
                                        div { class: "absolute inset-y-0 end-0 flex items-center pe-2.5 text-gray-500 dark:text-gray-400" }
                                        if let Some(err) = &form_state.get().errors.confirm_password {
                                            p { class: "mt-2 text-sm text-red-600 dark:text-red-500", "{err}" }
                                        }
                                    }
                                }
                            }
                            
                            // Terms checkbox
                            label { 
                                class: "flex items-center pt-1 text-sm font-medium text-gray-900 rtl:text-right dark:text-gray-300",
                                input {
                                    r#type: "checkbox",
                                    name: "accept",
                                    checked: form_state.get().data.accept_terms,
                                    onchange: move |e| handle_terms_change(e.value.parse().unwrap_or(false)),
                                    class: "me-2 h-4 w-4 rounded border-gray-300 bg-gray-100 text-primary-600 focus:ring-2 focus:ring-primary-500 dark:border-gray-500 dark:bg-gray-600 dark:ring-offset-gray-800 dark:focus:ring-primary-600"
                                }
                                span {
                                    "I accept the "
                                    Link { 
                                        to: "/terms", 
                                        class: "inline-flex items-center text-primary-600 hover:underline dark:text-primary-500",
                                        "Terms and Conditions"
                                    }
                                }
                                if let Some(err) = &form_state.get().errors.accept_terms {
                                    p { class: "mt-2 text-sm text-red-600 dark:text-red-500", "{err}" }
                                }
                            }
                            
                            // Submit button
                            button {
                                r#type: "submit",
                                disabled: form_state.get().loading,
                                class: "inline-flex items-center justify-center rounded-lg bg-primary-700 px-5 py-3 text-center text-base font-medium text-white focus-within:outline-none focus-within:ring-4 focus-within:ring-primary-300 hover:bg-primary-800 dark:bg-primary-600 dark:focus-within:ring-primary-800 dark:hover:bg-primary-700",
                                
                                if form_state.get().loading {
                                    rsx! { "Creating account..." }
                                } else {
                                    rsx! { "Create account" }
                                }
                            }
                            
                            // Login link
                            div { class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                "Already have an account? "
                                Link { 
                                    to: "/login", 
                                    class: "inline-flex items-center text-primary-600 hover:underline dark:text-primary-500",
                                    "Login here"
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

// Simulated API response types
enum RegisterResult {
    Success,
    Error(RegisterErrors),
}

// Simulated register API
async fn simulate_register_api(form: &RegisterForm) -> RegisterResult {
    // In a real app, this would be an actual API call
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    
    // Simple validation for demo purposes
    let mut errors = RegisterErrors::default();
    
    if form.name.is_empty() {
        errors.name = Some("Name is required".to_string());
    }
    
    if form.email.is_empty() || !form.email.contains('@') {
        errors.email = Some("Valid email is required".to_string());
    }
    
    if form.password.len() < 8 {
        errors.password = Some("Password must be at least 8 characters".to_string());
    }
    
    // If any errors, return them
    if errors.name.is_some() || errors.email.is_some() || errors.password.is_some() {
        return RegisterResult::Error(errors);
    }
    
    RegisterResult::Success
}