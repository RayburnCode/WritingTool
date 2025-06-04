// client/components/auth/common.rs
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AuthFormContainerProps {
    title: String,
    children: Element,
    footer: Element,
}

#[component]
pub fn AuthFormContainer(props: AuthFormContainerProps) -> Element {
    rsx! {
        div { class: "min-h-screen flex items-center justify-center bg-gray-50",
            div { class: "w-full max-w-md p-8 space-y-8 bg-white rounded-lg shadow",
                div { class: "text-center",
                    h2 { class: "text-2xl font-bold text-gray-900", "{props.title}" }
                }
                {props.children}
                {props.footer}
            }
        }
    }
}

#[derive(Props,  Clone, PartialEq)]
pub struct AuthInputFieldProps {
    id: String,
    label: String,
    r#type: String,
    value: String,
    disabled: bool,
    oninput: EventHandler<FormEvent>,
    error: Option<String>,
}

#[component]
pub fn AuthInputField(props: AuthInputFieldProps) -> Element {
    rsx! {
        div {
            label {
                r#for: props.id.clone(),
                class: "block text-sm font-medium text-gray-700",
                "{props.label}"
            }
            input {
                id: props.id.clone(),
                r#type: props.r#type.clone(),
                value: "{props.value}",
                disabled: props.disabled,
                oninput: move |e| props.oninput.call(e),
                class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500",
            }
            if let Some(err) = &props.error {
                p { class: "mt-1 text-sm text-red-600", "{err}" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AuthButtonProps {
    loading: bool,
    label: String,
    loading_label: String,
}

#[component]
pub fn AuthButton(props: AuthButtonProps) -> Element {
    rsx! {
        button {
            r#type: "submit",
            disabled: props.loading,
            class: "w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
            if props.loading {
                "{props.loading_label}"
            } else {
                "{props.label}"
            }
        }
    }
}

#[derive(Props,  Clone, PartialEq)]
pub struct AuthLinkProps {
    to: String,
    text: String,
    link_text: String,
}

#[component]
pub fn AuthLink(props: AuthLinkProps) -> Element {
    rsx! {
        div { class: "text-center text-sm",
            "{props.text} "
            Link {
                to: props.to.clone(),
                class: "font-medium text-indigo-600 hover:text-indigo-500",
                "{props.link_text}"
            }
        }
    }
}



#[derive(Props, Clone, PartialEq)]
pub struct AuthCheckboxProps {
    id: String,
    label: String,
    checked: bool,
    onchange: EventHandler<FormEvent>,
}

#[component]
pub fn AuthCheckbox(props: AuthCheckboxProps) -> Element {
    rsx! {
        div { class: "flex items-center",
            input {
                r#type: "checkbox",
                id: props.id.clone(),
                checked: props.checked,
                onchange: move |e| props.onchange.call(e),
                class: "h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded",
            }
            label {
                r#for: props.id.clone(),
                class: "ml-2 block text-sm text-gray-900",
                "{props.label}"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AuthInfoTextProps {
    text: String,
}

#[component]
pub fn AuthInfoText(props: AuthInfoTextProps) -> Element {
    rsx! {
        div { class: "text-sm text-gray-600", "{props.text}" }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AuthErrorAlertProps {
    message: String,
}

#[component]
pub fn AuthErrorAlert(props: AuthErrorAlertProps) -> Element {
    rsx! {
        div { class: "p-4 text-sm text-red-600 bg-red-50 rounded-md",
            p { "{props.message}" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AuthSuccessAlertProps {
    message: String,
}

#[component]
pub fn AuthSuccessAlert(props: AuthSuccessAlertProps) -> Element {
    rsx! {
        div { class: "p-4 text-sm text-green-600 bg-green-50 rounded-md",
            p { "{props.message}" }
        }
    }
}