#[cfg(target_os = "macos")]
pub fn set_traffic_lights_hidden(window: &mut gpui::Window, hidden: bool) {
    use objc2_app_kit::{NSView, NSWindowButton};
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    let Ok(raw_handle) = window.window_handle() else {
        return;
    };
    let RawWindowHandle::AppKit(appkit) = raw_handle.as_raw() else {
        return;
    };

    // The raw handle carries the NSView*; borrow it (NOT retained — gpui owns it,
    // and it outlives this call, so a temporary reference is sound).
    let ns_view: &NSView = unsafe { &*(appkit.ns_view.as_ptr() as *const NSView) };
    let Some(ns_window) = ns_view.window() else {
        return;
    };

    for kind in [
        NSWindowButton::CloseButton,
        NSWindowButton::MiniaturizeButton,
        NSWindowButton::ZoomButton,
    ] {
        if let Some(button) = ns_window.standardWindowButton(kind) {
            button.setHidden(hidden);
        }
    }
}
