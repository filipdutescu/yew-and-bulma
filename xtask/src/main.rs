use std::{
    fs::{create_dir_all, read_dir, remove_dir_all, remove_file},
    io,
    path::Path,
    process::{exit, Command, Stdio},
};

fn main() -> io::Result<()> {
    let mut show_cov = false;
    let mut verbose = false;
    for flag in std::env::args() {
        if flag == "--show" {
            show_cov = true;
        }
        if flag == "--verbose" || flag == "-v" {
            verbose = true;
        }
    }

    let coverage_dir_path = std::path::Path::new("coverage");
    if coverage_dir_path.is_dir() {
        remove_dir_all(coverage_dir_path)?;
    }
    create_dir_all(coverage_dir_path)?;

    println!("Gathering coverage from tests...");
    let mut report_cmd = Command::new("cargo");
    if verbose {
        report_cmd.stdout(Stdio::null());
    }
    report_cmd.env("CARGO_INCREMENTAL", "0");
    report_cmd.env("RUSTFLAGS", "-C instrument-coverage");
    report_cmd.env("LLVM_PROFILE_FILE", "coverage/test-%p-%m.profraw");
    report_cmd.arg("nextest");
    report_cmd.arg("run");
    report_cmd.arg("--all");
    report_cmd.arg("--all-targets");
    report_cmd.arg("--all-features");

    let exit_code = run_cmd!(report_cmd);
    if !exit_code.success() {
        println!("Error during coverage gathering: {exit_code}");
        exit(3)
    }

    let format = if show_cov { "html" } else { "lcov" };
    let output_file = format!("coverage/cov.{format}");
    println!("Generating coverage report at `{output_file}`...");

    let mut merge_reports_cmd = Command::new("grcov");
    if verbose {
        merge_reports_cmd.stdout(Stdio::null());
    }
    merge_reports_cmd.args([
        ".",
        "-s",
        ".",
        "--binary-path",
        "./target/debug/",
        "-t",
        format,
        "--branch",
        "--ignore-not-existing",
        "-o",
        &output_file,
    ]);

    let exit_code = run_cmd!(merge_reports_cmd);
    if !exit_code.success() {
        println!("Error during coverage report generation: {exit_code}");
        exit(4)
    }

    cleanup_raw_data(coverage_dir_path)?;
    if show_cov {
        let output_file = format!("{output_file}/index.html");
        if let Err(err) = open::that(output_file) {
            println!("Error while trying to show report: {err:?}");
            exit(5)
        }
    } else {
        println!("Coverage file found at {output_file}.");
    }

    Ok(())
}

fn cleanup_raw_data(coverage_dir_path: &Path) -> io::Result<()> {
    for f in read_dir(coverage_dir_path)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file())
    {
        if let Some(ext) = f.extension() {
            if ext == "profraw" {
                if let Err(err) = remove_file(&f) {
                    let file_path = f.display();
                    println!("WARN: Failed to remove raw coverage file {file_path}: {err}");
                }
            }
        }
    }

    Ok(())
}

#[macro_export]
macro_rules! run_cmd {
    ($cmd:ident) => {{
        let mut process = match $cmd.spawn() {
            Ok(process) => process,
            Err(err) => {
                println!(
                    "Failed to spawn process `{}`: {:?}",
                    $cmd.get_program()
                        .to_os_string()
                        .into_string()
                        .ok()
                        .unwrap_or("invalid characters in program name".to_owned()),
                    err
                );
                exit(1)
            }
        };
        let output = match process.wait() {
            Ok(output) => output,
            Err(err) => {
                println!("Failed to wait for output: {err:?}");
                exit(2)
            }
        };

        output
    }};
}
