// GPUI did not support hiding traffic ligt ui yet
#[cfg(target_os = "macos")]
pub fn set_traffic_lights_hidden(window: &mut gpui::Window, hidden: bool) {
    use objc::{msg_send, sel, sel_impl};
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    let Ok(raw_handle) = window.window_handle() else {
        return;
    };
    let RawWindowHandle::AppKit(appkit) = raw_handle.as_raw() else {
        return;
    };

    // The raw handle contains ns_view (NSView*); get NSWindow from it
    let ns_view = appkit.ns_view.as_ptr() as cocoa::base::id;

    unsafe {
        use cocoa::appkit::NSWindowButton;
        let ns_window: cocoa::base::id = msg_send![ns_view, window];
        if ns_window.is_null() {
            return;
        }
        for btn in [
            NSWindowButton::NSWindowCloseButton,
            NSWindowButton::NSWindowMiniaturizeButton,
            NSWindowButton::NSWindowZoomButton,
        ] {
            let button: cocoa::base::id = msg_send![ns_window, standardWindowButton: btn];
            if !button.is_null() {
                let _: () = msg_send![button, setHidden: hidden];
            }
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub fn set_traffic_lights_hidden(_window: &mut gpui::Window, _hidden: bool) {}
