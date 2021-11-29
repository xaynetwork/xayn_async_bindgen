pub(crate) fn abort_if_dirty() {
    #[cfg(not(test))]
    {
        proc_macro_error::abort_if_dirty()
    }
    // in tests we fail fast so it's never dirty
}

macro_rules! __abort {
    ($span:expr, $msg:expr) => (
        #[cfg(test)]
        {
            panic!("{}", $msg);
        }
        #[cfg(not(test))]
        {
            proc_macro_error::abort!($span, $msg)
        }
    );
}

pub(crate)use __abort as abort;

macro_rules! __emit_error {
    ($span:expr, $msg:expr => $cont:expr) => (
        #[cfg(test)]
        {
            panic!("{}", $msg);
        }
        #[cfg(not(test))]
        {
            proc_macro_error::emit_error!($span, $msg);
            $cont
        }
    );
}

pub(crate)use __emit_error as emit_error;
