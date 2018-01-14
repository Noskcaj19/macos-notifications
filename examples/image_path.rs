extern crate macos_notifications;

use macos_notifications::{Notification, NotificationImage};

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
    Notification::new()
        .title("Image")
        .content_image(NotificationImage::File(image1.to_str().unwrap()))
        .app_image(NotificationImage::File(image2.to_str().unwrap()))
        .deliver();
}
