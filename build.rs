use cc;

fn main() {
    cc::Build::new().file("src/link.c").compile("fnmatch")
}
