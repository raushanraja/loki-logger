use std::collections::HashMap;

#[macro_export]
macro_rules! tinfo {
    (target: $target:expr, $($arg:tt)+) => (log::log!(target: $target,log:: Level::Info, $($arg)+));
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => ({
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs().to_string().as_str();
        log::log!(target: "", log::Level::Info , now=now, $($key $(:$capture)? $(= $value)?),+; $($arg)+);
    });

    ($($arg:tt)+) => ({
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs().to_string();
        log::log!(target: "", log::Level::Info , now=now.as_str(); $($arg)+);
    });
}

#[macro_export]
macro_rules! twarn {
    (target: $target:expr, $($arg:tt)+) => (log::log!(target: $target, log::Level::Warn, $($arg)+));
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => ({
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs().to_string();
        log::log!(target: "", log::Level::Warn , now=now.as_str(), $($key $(:$capture)? $(= $value)?),+; $($arg)+);
    });

    ($($arg:tt)+) => ({
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs().to_string();
        log::log!(target: "", log::Level::Warn , now=now.as_str(); $($arg)+);
    });
}

#[macro_export]
macro_rules! terror {
    (target: $target:expr, $($arg:tt)+) => (log::log!(target: $target, log::Level::Error, $($arg)+));
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => ({
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs().to_string();
        log::log!(target: "", log::Level::Error , now=now.as_str(), $($key $(:$capture)? $(= $value)?),+; $($arg)+);
    });

    ($($arg:tt)+) => ({
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs().to_string();
        log::log!(target: "", log::Level::Error , now=now.as_str(); $($arg)+);
    });
}

#[macro_export]
macro_rules! tdebug {
    (target: $target:expr, $($arg:tt)+) => (log::log!(target: $target, log::Level::Debug, $($arg)+));
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => ({
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs().to_string();
        log::log!(target: "", log::Level::Debug , now=now.as_str(), $($key $(:$capture)? $(= $value)?),+; $($arg)+);
    });

    ($($arg:tt)+) => ({
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs().to_string();
        log::log!(target: "", log::Level::Debug , now=now.as_str(); $($arg)+);
    });
}

pub fn logger_init() {
    let app_name = env!("CARGO_PKG_NAME").to_string();
    let app_version = env!("CARGO_PKG_VERSION").to_string();

    let initial_labels: HashMap<String, String> = HashMap::from_iter([
        ("application".to_string(), app_name.clone()),
        ("version".to_string(), app_version.clone()),
    ]);

    loki_logger::init_with_labels(
        "http://localhost:3100/loki/api/v1/push",
        log::LevelFilter::Info,
        initial_labels,
    )
    .expect("Failed to initialize logger");
}

#[tokio::main]
async fn main() {
    logger_init();

    for i in 0..10 {
        tinfo!("Hello, world! {}", i);
        twarn!("Hello, world! {}", i);
        terror!("Hello, world! {}", i);
        tdebug!("Hello, world! {}", i);
    }

    #[allow(clippy::empty_loop)]
    loop {}
}
