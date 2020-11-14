#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_symbols_defined() {
        unsafe { assert!(FNM_NOMATCH > 0 || FNM_NOMATCH <= 0) };
        unsafe { assert!(FNM_NOESCAPE > 0 || FNM_NOESCAPE <= 0) };
        unsafe { assert!(FNM_PATHNAME > 0 || FNM_PATHNAME <= 0) };
        unsafe { assert!(FNM_PERIOD > 0 || FNM_PERIOD <= 0) };
        let _ = |pattern: *const c_char, string: *const c_char, flags: c_int| unsafe {
            fnmatch(pattern, string, flags)
        };
    }
}

use std::os::raw::{c_char, c_int};

extern "C" {
    /// Match failed.
    #[link_name = "fnm_nomatch"]
    pub static FNM_NOMATCH: c_int;
    /// Disable backslash escpaing.
    #[link_name = "fnm_noescape"]
    pub static FNM_NOESCAPE: c_int;
    /// Slash must be matched by slash.
    #[link_name = "fnm_pathname"]
    pub static FNM_PATHNAME: c_int;
    /// Period must be matched by period.
    #[link_name = "fnm_period"]
    pub static FNM_PERIOD: c_int;
    /// The `fnmatch()` function checks whether the string argument matches the
    /// pattern argument, which is a shell wildcard pattern (see glob(7)).
    ///
    /// The flags argument modifies the behavior; it is the bitwise OR of zero
    /// or more of the following flags:
    pub fn fnmatch(pattern: *const c_char, string: *const c_char, flags: c_int) -> c_int;
}
