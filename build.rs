use winres;

fn main() {
    if cfg!(target_os = "windows") {
        winres::WindowsResource::new()
            .set_icon("test.ico")
            .compile()
            .unwrap();
    }
}
