#[clippy::format_args]
macro_rules! format_and_log {
    ($lvl:expr, $($arg:tt)+) => ({
        let msg = format!($($arg)+);

        log::log!(

            $lvl,

            $($arg)+

        );

        msg
    });
}

pub(crate) use format_and_log;

#[clippy::format_args]
macro_rules! error {
    ($($arg:tt)+) => (crate::helpers::format_and_log!(log::Level::Error, $($arg)+))
}
pub(crate) use error;

#[clippy::format_args]
macro_rules! warns {
    ($($arg:tt)+) => (crate::helpers::format_and_log!(log::Level::Warning, $($arg)+))
}
pub(crate) use warns;

#[clippy::format_args]
macro_rules! info {
    ($($arg:tt)+) => (crate::helpers::format_and_log!(log::Level::Info, $($arg)+))
}
pub(crate) use info;

#[clippy::format_args]
macro_rules! debug {
    ($($arg:tt)+) => (crate::helpers::format_and_log!(log::Level::Debug, $($arg)+))
}
pub(crate) use debug;
