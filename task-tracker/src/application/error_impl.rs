use crate::domain::error::AppError;
use std::{error::Error as StdError, format, io::ErrorKind as IoErrorKind, write};

const INVALIDINPUT: IoErrorKind = IoErrorKind::InvalidInput;
const CONNECTIONABORTED: IoErrorKind = IoErrorKind::ConnectionAborted;
const NOTFOUND: IoErrorKind = IoErrorKind::NotFound;

// #[derive(Debug)]
// #[allow(dead_code)]
// struct Error<'a, T> {
//     code: &'a str,
//     kind: IoErrorKind,
//     message: T,
// }

use std::fmt::{Display, Formatter};
impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let error_str = match self {
            AppError::InvalidInput(name, message) => get_error_str(name, INVALIDINPUT, message),
            AppError::NotFound(name, message) => get_error_str(name, NOTFOUND, message),
            AppError::Aborted(name, message) => get_error_str(name, CONNECTIONABORTED, message),
        };
        write!(f, "{}", error_str)
    }
}

impl StdError for AppError {}

fn get_error_str<'a, T: std::fmt::Debug>(name: &'a str, kind: IoErrorKind, message: T) -> String {
    format!(
        "Error\n------------------\ncode   : {}\nkind   : {:?}\nmessage: {:?}\n++++++++++++++++++",
        name, kind, message,
    )
}

// > It's me, not Github Copilot (AI)!
// fungsi (bukan impl...for...): untuk memberitahukan jika ada pesan error yang diinput salah
// => function (not impl...for...): to notify if an error message for an invalid input error

// > It's me, not Github Copilot (AI)!
// fungsi: untuk memberitahukan bahwa jika pesan error yang input tidak ditemukan
// => function: to notify that if an error message for a not found input error

// fungsi (bukan impl...for...): untuk memberitahukan jika ada pesan error yang diinput salah
// => function (not impl...for...): to notify if an error message for an invalid input error
pub fn panic_invalid_input<T: std::fmt::Debug>(name: &'static str, message: T) -> () {
    let err_file = get_error_str(name, INVALIDINPUT, message);
    panic!("{err_file:?}");
    // // TODO: testing to uncomment this line to exit the process on panic
    // eprintln!("{err_file:?}");
    // process::exit(0);
}

// fungsi: untuk memberitahukan bahwa jika pesan error yang input tidak ditemukan
// => function: to notify that if an input error message is not found
pub fn panic_not_found_input<T: std::fmt::Debug>(name: &'static str, message: T) -> () {
    let err_file = get_error_str(name, NOTFOUND, message);
    panic!("{err_file:?}");
    // // TODO: uncomment this line to exit the process on panic
    // eprintln!("{err_file:?}");
    // process::exit(0);
}
