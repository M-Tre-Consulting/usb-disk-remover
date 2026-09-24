fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_style("fluent".to_string());
    slint_build::compile_with_config("ui/app.slint", config).expect("Slint compilation failed");

    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icons/icon.ico");
        let _ = res.compile();
    }
}
