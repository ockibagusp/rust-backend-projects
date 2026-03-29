use std::io::{Error as IoError, ErrorKind as IoErrorKind};

#[derive(Debug)]
#[allow(dead_code)]
struct Error<'a, T> {
    code: &'a str,
    kind: std::io::ErrorKind,
    message: T,
}

fn get_error_func<'a, T>(name: &'a str, kind: std::io::ErrorKind, message: T) -> Error<'a, T> {
    Error {
        code: name,
        kind,
        message,
    }
}

const INVALIDINPUT: IoErrorKind = std::io::ErrorKind::InvalidInput;
const NOTFOUND: IoErrorKind = std::io::ErrorKind::NotFound;

// > It's me, not Github Copilot (AI)!
// fungsi (bukan impl...for...): untuk memberitahukan jika ada pesan error yang diinput salah
// => function (not impl...for...): to notify if an error message for an invalid input error
pub fn error_invalid_input<T: std::fmt::Debug>(name: &'static str, message: &str) -> IoError {
    let err_file = get_error_func(name, INVALIDINPUT, message);
    return IoError::new(INVALIDINPUT, format!("{:?}", err_file));
}

pub fn error_invalid_input_str<S>(name: &'static str, message: S) -> IoError
where
    S: AsRef<str>,
{
    let err_file = get_error_func(name, INVALIDINPUT, message.as_ref());
    return IoError::new(INVALIDINPUT, format!("{:?}", err_file));
}

// > It's me, not Github Copilot (AI)!
// fungsi: untuk memberitahukan bahwa jika pesan error yang input tidak ditemukan
// => function: to notify that if an error message for a not found input error
pub fn error_not_found_input<T: std::fmt::Debug>(name: &'static str, message: &str) -> IoError {
    let err_file = get_error_func(name, NOTFOUND, message);
    return IoError::new(IoErrorKind::NotFound, format!("{:?}", err_file));
}

pub fn error_kind<T: std::fmt::Debug>(name: &'static str, err: IoError) -> IoError {
    let err_file = get_error_func(name, INVALIDINPUT, format!("{}", err));
    return IoError::new(err.kind(), format!("{:?}", err_file));
}

pub fn error_kind_refused<T: std::fmt::Debug>(name: &'static str, message: &str) -> IoError {
    let err_file = get_error_func(name, IoErrorKind::ConnectionRefused, message);
    return IoError::new(IoErrorKind::ConnectionRefused, format!("{:?}", err_file));
}

pub fn error_kind_aborted<T: std::fmt::Debug>(name: &'static str, message: &str) -> IoError {
    let err_file = get_error_func(name, IoErrorKind::ConnectionAborted, message);
    return IoError::new(IoErrorKind::ConnectionAborted, format!("{:?}", err_file));
}

// fungsi (bukan impl...for...): untuk memberitahukan jika ada pesan error yang diinput salah
// => function (not impl...for...): to notify if an error message for an invalid input error
pub fn panic_invalid_input<T: std::fmt::Debug>(name: &'static str, message: T) -> () {
    let err_file = get_error_func(name, INVALIDINPUT, message);
    panic!("{err_file:?}");
    // // TODO: testing to uncomment this line to exit the process on panic
    // eprintln!("{err_file:?}");
    // process::exit(0);
}

// fungsi: untuk memberitahukan bahwa jika pesan error yang input tidak ditemukan
// => function: to notify that if an input error message is not found
pub fn panic_not_found_input<T: std::fmt::Debug>(name: &'static str, message: T) -> () {
    let err_file = get_error_func(name, std::io::ErrorKind::NotFound, message);
    panic!("{err_file:?}");
    // // TODO: uncomment this line to exit the process on panic
    // eprintln!("{err_file:?}");
    // process::exit(0);
}
