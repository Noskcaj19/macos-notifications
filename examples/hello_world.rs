extern crate macos_notifications;

fn main() {
    // Call init so we get notifcations
    macos_notifications::init();

    // Construct and send a new notification
    macos_notifications::Notification::new()
        .title("Notification")
        .body("Hello, World!")
        .deliver();
}
