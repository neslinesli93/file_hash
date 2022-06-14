use serde::Deserialize;
use std::path::Path;
use std::process::Command;

const BLANK_PDF_PATH: &str = "/tmp/blank.pdf";

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Deserialize)]
pub struct Resolution {
    pub density: u32,
    pub height: u32,
}

pub fn create_blank_pdf() {
    if !Path::new(BLANK_PDF_PATH).exists() {
        let output = Command::new("mutool")
            .arg("create")
            .arg("-o")
            .arg(BLANK_PDF_PATH)
            .arg("/dev/null")
            .output()
            .expect("Failed to create blank pdf");

        assert!(output.status.success());
    }
}

pub fn create_blank_png(resolution: &Resolution) {
    let filename = format!(
        "/tmp/blank-{}-{}.png",
        resolution.density, resolution.height
    );

    let output = Command::new("mutool")
        .arg("convert")
        .arg("-O")
        .arg(format!(
            "resolution={},height={}",
            resolution.density, resolution.height
        ))
        .arg("-o")
        .arg(filename)
        .arg(BLANK_PDF_PATH)
        .output()
        .expect("Failed to create blank png");

    assert!(output.status.success());
}

pub fn check_images(filename: String) -> bool {
    let output = Command::new("mutool")
        .arg("info")
        .arg(filename)
        .output()
        .expect("Failed to check for images");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains("Images (")
}
