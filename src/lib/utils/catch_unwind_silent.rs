use std::{
    panic::{catch_unwind, set_hook, take_hook, UnwindSafe},
    thread::Result,
};

pub fn catch_unwind_silent<F, T>(f: F) -> Result<T>
where
    F: FnOnce() -> T + UnwindSafe, {
    let orig_hook = take_hook();
    set_hook(Box::new(|_| {}));
    let res = catch_unwind(f);
    set_hook(orig_hook);
    res
}
