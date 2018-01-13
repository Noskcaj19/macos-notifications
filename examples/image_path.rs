extern crate macos_notifications;

use std::env;
use std::path::PathBuf;

fn get_dir() -> PathBuf {
    let mut dir = env::current_dir().unwrap();
    dir.push("examples");
    dir
}

fn main() {
    let dir = get_dir();
    let image1 = dir.join("rust.png");
    let image2 = dir.join("ferris.png");

    // Call init so we get notifcations
    macos_notifications::init();

    // Construct and send a new notification
    macos_notifications::Notification::new()
        .title("Image")
        .content_image_path(image1.to_str().unwrap())
        .app_image_path(image2.to_str().unwrap())
        .deliver();
}
