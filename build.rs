use cc;

fn main() {
    cc::Build::new()
        .define(
            "FNMATCH_SYS_MAYBE_CASEFOLD",
            if cfg!(feature = "maybe-casefold") {
                "1"
            } else {
                "0"
            },
        )
        .define(
            "FNMATCH_SYS_MAYBE_EXTMATCH",
            if cfg!(feature = "maybe-extmatch") {
                "1"
            } else {
                "0"
            },
        )
        .file("src/link.c")
        .compile("fnmatch");
}
