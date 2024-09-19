# fnmatch-sys

Provides a rust ffi link to the `C` header file `fnmatch.h`. According to the
rust convention, no effort is made to add safe abstractions to the declarations
provided.

## Building

`fnmatch.h` is required in `CC`'s build path to link the library.

## Declarations provided

```rust
    pub static FNM_NOMATCH: c_int; // Match failed.

    pub static FNM_NOESCAPE: c_int; // Disable backslash escpaing.

    pub static FNM_PATHNAME: c_int; // Slash must be matched by slash.

    pub static FNM_PERIOD: c_int; // Period must be matched by period.

    pub static FNM_CASEFOLD: c_int; // A GNU extension: the pattern is matched ignoring a case.

    pub fn fnmatch(pattern: *const c_char, string: *const c_char, flags: c_int) -> c_int;
```
