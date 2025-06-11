<!-- @format -->

1. Theme (Dark/Light Mode)

- Store the current theme preference (Theme::Dark or Theme::Light).

2. Sidebar/Collapsible Menu State

- Track whether a sidebar is open or collapsed.

3. Modal/Dialog Visibility

- Control which modal or dialog is currently open (e.g., LoginModal, SettingsDialog).

4. User Profile

- Store the logged-in user's data (username, email, role, etc.).

5. Form State

- Track form inputs (e.g., a NewPostForm or UserSettingsForm) before submission.

6. Toast Messages

- Queue notifications (e.g., Vec<Toast>) to show success/error messages.

7. Loading States

- Track ongoing async operations (e.g., is_loading: bool).
