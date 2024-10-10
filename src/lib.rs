#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_symbols_defined() {
        unsafe { assert!(FNM_NOMATCH > 0 || FNM_NOMATCH <= 0) };
        unsafe { assert!(FNM_NOESCAPE > 0 || FNM_NOESCAPE <= 0) };
        unsafe { assert!(FNM_PATHNAME > 0 || FNM_PATHNAME <= 0) };
        unsafe { assert!(FNM_PERIOD > 0 || FNM_PERIOD <= 0) };
        #[cfg(feature = "gnu")]
        unsafe {
            assert!(FNM_CASEFOLD > 0 || FNM_CASEFOLD <= 0);
            assert!(FNM_EXTMATCH > 0 || FNM_EXTMATCH <= 0);
        }
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
    ///
    /// - `FNM_NOESCAPE`
    /// - `FNM_PATHNAME`
    /// - `FNM_PERIOD`
    /// - `FNM_CASEFOLD` (feature = "gnu")
    /// - `FNM_EXTMATCH` (feature = "gnu")
    ///
    /// RETURN VALUE
    /// Zero if string matches pattern, `FNM_NOMATCH` if there is no match or
    /// another nonzero value if there is an error.
    pub fn fnmatch(pattern: *const c_char, string: *const c_char, flags: c_int) -> c_int;
}

// GNU Extensions to fnmatch.h
#[cfg(feature = "gnu")]
extern "C" {
    /// Case insensitive search.
    #[link_name = "fnm_casefold"]
    pub static FNM_CASEFOLD: c_int;
    /// If this flag (a GNU extension) is set, extended patterns are supported, as
    /// introduced by 'ksh' and now supported by other shells.  The extended format is as
    /// follows, with pattern-list being a '|' separated list of patterns.
    ///
    /// '?(pattern-list)' - The pattern matches if zero or one occurrences of any of the
    /// patterns in the pattern-list match the input string.
    ///
    /// '*(pattern-list)' - The pattern matches if zero or more occurrences of any of the
    /// patterns in the pattern-list match the input string.
    ///
    /// '+(pattern-list)' - The pattern matches if one or more occurrences of any of the
    /// patterns in the pattern-list match the input string.
    ///
    /// '@(pattern-list)' - The pattern matches if exactly one occurrence of any of the
    /// patterns in the pattern-list match the input string.
    ///
    /// '!(pattern-list)' - The pattern matches if the input string cannot be matched with
    /// any of the patterns in the pattern-list.
    #[link_name = "fnm_extmatch"]
    pub static FNM_EXTMATCH: c_int;
}
