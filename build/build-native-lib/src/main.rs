// Note the lack of the `#[link]` attribute. We’re delegating the responsibility
// of selecting what to link over to the build script rather than hard-coding
// it in the source file.
extern "C" {
    fn hello();
}

fn main() {
    unsafe {
        hello();
    }
}
