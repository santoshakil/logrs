use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

#[no_mangle]
pub extern "C" fn log_e(message: *const libc::c_char) {
    let message = unsafe { std::ffi::CStr::from_ptr(message) };
    let message = message.to_str().unwrap().to_string();
    log::error!("{}", message);
}

#[no_mangle]
pub extern "C" fn log_w(message: *const libc::c_char) {
    let message = unsafe { std::ffi::CStr::from_ptr(message) };
    let message = message.to_str().unwrap().to_string();
    log::warn!("{}", message);
}

#[no_mangle]
pub extern "C" fn log_i(message: *const libc::c_char) {
    let message = unsafe { std::ffi::CStr::from_ptr(message) };
    let message = message.to_str().unwrap().to_string();
    log::info!("{}", message);
}

#[no_mangle]
pub extern "C" fn init_logrs(path: *const libc::c_char) -> libc::c_int {
    let path = unsafe { std::ffi::CStr::from_ptr(path) };
    let path = path.to_str().unwrap().to_string();
    match init_logrs_r(path) {
        Ok(_) => 0,
        _ => 1,
    }
}

fn init_logrs_r(path: String) -> Result<(), std::io::Error> {
    let p = PatternEncoder::new("{d(%H:%M:%S)} - {l} - {m}\n");
    let stderr = ConsoleAppender::builder().encoder(Box::new(p)).build();

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
        .build(path)
        .unwrap();

    let level = log::LevelFilter::Info; // Trace < Debug < Info < Warn < Error
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("stderr", Box::new(stderr)))
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(level),
        )
        .unwrap();

    if let Err(err) = log4rs::init_config(config) {
        log::info!("Error initializing logrs: {}", err);
    }

    log::info!("Logrs initialized.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logrs() {
        let path = std::ffi::CString::new("./logs.log").unwrap();
        let path = path.as_ptr();
        assert_eq!(init_logrs(path), 0);

        let message = std::ffi::CString::new("test").unwrap();
        let message = message.as_ptr();

        assert_eq!(log_i(message), ());
        assert_eq!(log_w(message), ());
        assert_eq!(log_e(message), ());
    }
}
