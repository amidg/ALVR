pub fn init_logging() {
    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("ALVR native-rust")
            .with_min_level(log::Level::Info),
    );

    crate::logging::set_panic_hook();
}
