use std::fmt::Debug;

pub trait CoreResultExt {
    fn or_log_debug(&self);
}

impl<T: Debug, E: Debug> CoreResultExt for Result<T, E> {
    fn or_log_debug(&self) {
        if self.is_err() {
            dbg!(self);
        }
    }
}
