use std::process::Command;

fn main() {
    let file = std::fs::read_to_string("guide/src/libs_python.txt").unwrap();
    for line in file.lines() {
        //let line = line.unwrap();
        let mut parts = line.split(' ');
        let package_name = parts.next().unwrap();
        let expected_sha256 = parts.next().expect("Missing SHA-256 in line");

        let output = Command::new("pip")
            .arg("show")
            .arg(package_name)
            .output()
            .expect("Failed to execute pip command");

        println!("Output ---> {:?}", output);

        if output.status.success() {
            let installed_sha256 =
                extract_sha256_from_pip_output(std::str::from_utf8(&output.stdout).unwrap());

            println!("found -> {installed_sha256}");

            if installed_sha256 == expected_sha256 {
                println!("{} is installed with the correct SHA-256", package_name);
            } else {
                println!(
                    "{} is installed, but the SHA-256 does not match (expected: {}, installed: {})",
                    package_name, expected_sha256, installed_sha256
                );
            }
        } else {
            println!("{} is not installed", package_name);
        }
    }
}

fn extract_sha256_from_pip_output(output: &str) -> String {
    for line in output.lines() {
        if line.starts_with("SHA256 checksum:") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                return parts[1].trim().to_string(); // Extract and trim the SHA-256
            }
        }
    }

    // If not found, handle appropriately (e.g., return an empty string or panic)
    "".to_string()
}
