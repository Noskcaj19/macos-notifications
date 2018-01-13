extern crate macos_notifications;

fn main() {
    let image_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/200px-Rust_programming_language_black_logo.svg.png";

    // Call init so we get notifcations
    macos_notifications::init();

    // Construct and send a new notification
    macos_notifications::Notification::new()
        .title("Image")
        .app_image_url(image_url)
        .deliver();
}
