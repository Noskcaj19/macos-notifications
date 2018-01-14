//! MacOS notifcations in Rust
//!
//! Provides access to the NSUserNotification APIs

extern crate cocoa;
#[macro_use]
extern crate objc;

use objc::declare::ClassDecl;
use objc::runtime::{self, Class, Method, Object, Sel};

use cocoa::base::nil;

use cocoa::foundation::NSString;

/// Setup function to swizzle NSBundle to let notification be delivered without bundling
/// Must be called before any notifcations will be delivered, only needs to be called once
///
/// Returns true if swizzling was successful, otherwise returns false
pub fn init() -> bool {
    // Create new class
    let superclass = Class::get("NSObject").unwrap();
    let mut decl = ClassDecl::new("NSBundleOverride", superclass).unwrap();

    extern "C" fn bundle_identifier_override(_: &Object, _cmd: Sel) -> *mut Object {
        unsafe { NSString::alloc(nil).init_str("com.apple.Terminal") }
    }
    unsafe {
        decl.add_method(
            sel!(__bundleIdentifier),
            bundle_identifier_override as extern "C" fn(&Object, Sel) -> *mut Object,
        );
    }
    decl.register();

    // Real swizzling part
    let cls = Class::get("NSBundle").unwrap();
    unsafe {
        let bi_original =
            runtime::class_getInstanceMethod(cls, Sel::register("bundleIdentifier")) as *mut Method;
        let custom_cls = Class::get("NSBundleOverride").unwrap();
        let bi_override =
            runtime::class_getInstanceMethod(custom_cls, Sel::register("__bundleIdentifier"))
                as *mut Method;

        // And now we swizzle
        runtime::method_exchangeImplementations(bi_original, bi_override);
    }

    // Check our work
    unsafe {
        let main_bundle: *mut Object = msg_send![cls, mainBundle];
        let id: *mut Object = msg_send![main_bundle, bundleIdentifier];
        return id.isEqualToString("com.apple.Terminal");
    }
}

/// Possible image sources for `Notification`
pub enum NotificationImage<'a> {
    /// A remote resource
    Url(&'a str),
    /// A local resource on the disk
    File(&'a str),
}

/// A Desktop notifiction
///
/// Configurable via builder pattern before being delivered by `deliver()`
pub struct Notification<'a> {
    title: Option<&'a str>,
    subtitle: Option<&'a str>,
    body: Option<&'a str>,
    content_image: Option<NotificationImage<'a>>,
    app_image: Option<NotificationImage<'a>>,
}

impl<'a> Notification<'a> {
    /// Creates a new notification with no fields set
    pub fn new() -> Notification<'a> {
        Notification {
            title: None,
            subtitle: None,
            body: None,
            content_image: None,
            app_image: None,
        }
    }

    /// Sets the title of the notification
    pub fn title(&mut self, title: &'a str) -> &mut Notification<'a> {
        self.title = Some(title);
        self
    }

    /// Sets the subtitle of the notification
    pub fn subtitle(&mut self, subtitle: &'a str) -> &mut Notification<'a> {
        self.subtitle = Some(subtitle);
        self
    }

    /// Sets the body text of the notification
    pub fn body(&mut self, body: &'a str) -> &mut Notification<'a> {
        self.body = Some(body);
        self
    }

    /// Sets the notification content image to the contents of local path, this is the image on the right of the notification.
    pub fn content_image_path(&mut self, path: &'a str) -> &mut Notification<'a> {
        self.content_image = Some(NotificationImage::File(path));
        self
    }

    /// Sets the notification content image to the contents of a url, this is the image on the right of the notification.
    pub fn content_image_url(&mut self, url: &'a str) -> &mut Notification<'a> {
        self.content_image = Some(NotificationImage::Url(url));
        self
    }

    /// Sets the notification app image to the contents of a local path, this is the image on the left of the notification.
    /// This uses private APIs and may break
    pub fn app_image_path(&mut self, path: &'a str) -> &mut Notification<'a> {
        self.app_image = Some(NotificationImage::File(path));
        self
    }

    /// Sets the notification app image to the contents of a url, this is the image on the left of the notification.
    /// This uses private APIs and may break
    pub fn app_image_url(&mut self, url: &'a str) -> &mut Notification<'a> {
        self.app_image = Some(NotificationImage::Url(url));
        self
    }

    /// Deliver the constructed notifcation immediately
    pub fn deliver(&self) {
        let notification_cls = Class::get("NSUserNotification").unwrap();
        let center = Class::get("NSUserNotificationCenter").unwrap();
        unsafe {
            let notification: *mut Object = msg_send![notification_cls, alloc];
            let notification: *mut Object = msg_send![notification, init];

            if let Some(title) = self.title {
                msg_send![notification, setTitle:NSString::alloc(nil).init_str(title)];
            }
            if let Some(subtitle) = self.subtitle {
                msg_send![notification, setSubtitle:NSString::alloc(nil).init_str(subtitle)];
            }
            if let Some(body) = self.body {
                msg_send![notification, setInformativeText:NSString::alloc(nil).init_str(body)];
            }
            if let Some(ref image_data) = self.content_image {
                let img_cls = Class::get("NSImage").unwrap();
                let image: *mut Object = msg_send![img_cls, alloc];

                let image: *mut Object = match image_data {
                    &NotificationImage::File(file) => msg_send![
                            image,
                            initWithContentsOfFile: NSString::alloc(nil).init_str(file)
                        ],
                    &NotificationImage::Url(url) => {
                        let url_cls = Class::get("NSURL").unwrap();
                        let nsurl: *mut Object = msg_send![
                            url_cls,
                            URLWithString:NSString::alloc(nil).init_str(url)
                        ];
                        msg_send![image, initWithContentsOfURL: nsurl]
                    }
                };
                msg_send![notification, setContentImage: image];
            }
            if let Some(ref image_data) = self.app_image {
                let img_cls = Class::get("NSImage").unwrap();
                let image: *mut Object = msg_send![img_cls, alloc];

                let image: *mut Object = match image_data {
                    &NotificationImage::File(file) => msg_send![
                            image,
                            initWithContentsOfFile: NSString::alloc(nil).init_str(file)
                        ],
                    &NotificationImage::Url(url) => {
                        let url_cls = Class::get("NSURL").unwrap();
                        let nsurl: *mut Object =
                            msg_send![url_cls, URLWithString:NSString::alloc(nil).init_str(url)];
                        msg_send![image, initWithContentsOfURL: nsurl]
                    }
                };
                msg_send![notification, setValue: image forKey:NSString::alloc(nil).init_str("_identityImage")];
            }
            let default_center: *mut Object = msg_send![center, defaultUserNotificationCenter];
            msg_send![default_center, deliverNotification: notification];

            msg_send![notification, release];
            self.runloop();
        }
    }

    // TODO: Use delegate to only loop as needed
    fn runloop(&self) {
        let runloop_cls = Class::get("NSRunLoop").unwrap();
        let date_cls = Class::get("NSDate").unwrap();
        unsafe {
            let current_run_loop: *mut Object = msg_send![runloop_cls, currentRunLoop];
            let till_date: *mut Object = msg_send![date_cls, dateWithTimeIntervalSinceNow:0.2];
            msg_send![current_run_loop, runUntilDate: till_date];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Notification;

    #[test]
    fn init() {
        super::init();
    }

    #[test]
    fn title() {
        let mut note = Notification::new();

        note.title("A title");
        note.deliver();
    }

    #[test]
    fn subtitle() {
        let mut note = Notification::new();

        note.title("A title");
        note.subtitle("Subtitle content");
        note.deliver();
    }

    #[test]
    fn body() {
        let mut note = Notification::new();

        note.title("A title");
        note.subtitle("Subtitle content");
        note.body("Body content");
        note.deliver();
    }

    // The actual resource does not have to exist, we are only testing for segfaults
    #[test]
    fn content_img_path() {
        let mut note = Notification::new();

        note.content_image_path("/");
        note.deliver();
    }

    #[test]
    fn content_image_url() {
        let mut note = Notification::new();

        note.content_image_url("https://google.com");
        note.deliver();
    }

    #[test]
    fn app_img_path() {
        let mut note = Notification::new();

        note.app_image_path("/");
        note.deliver();
    }

    #[test]
    fn app_img_url() {
        let mut note = Notification::new();

        note.app_image_url("https://google.com");
        note.deliver();
    }
}
