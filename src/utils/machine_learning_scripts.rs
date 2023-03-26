use std::process::Command;

pub fn image_to_text(image_url: String) -> String {

    // Call Python script with image path as argument and capture output
    let output = Command::new("python")
        .arg("python_scripts/image_to_text.py")
        .arg(image_url)
        .output()
        .expect("Failed to execute command");

    // Convert output to string and return
    let text = str::from_utf8(&output.stdout).unwrap().trim().to_owned();
    text
}
