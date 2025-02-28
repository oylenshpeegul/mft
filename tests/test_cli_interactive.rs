// The interactive tests are in a separate file,
// since they use `rexpect`, which internally uses quirky fork semantics to open a pty.
// They will fail if tried to be executed concurrently any other CLI test.

mod fixtures;

use fixtures::*;

use std::fs::File;
use std::io::{Read, Write};
use tempfile::tempdir;

use assert_cmd::cargo::cargo_bin;
#[cfg(not(target_os = "windows"))]
use rexpect::spawn;

// It should behave the same on windows, but interactive testing relies on unix pty internals.
#[test]
#[cfg(not(target_os = "windows"))]
fn test_it_confirms_before_overwriting_a_file() {
    let d = tempdir().unwrap();
    let f = d.as_ref().join("test.out");

    let mut file = File::create(&f).unwrap();
    file.write_all(b"I'm a file!").unwrap();

    let sample = mft_sample();

    let cmd_string = format!(
        "{bin} -f {output_file} {sample}",
        bin = cargo_bin("mft_dump").display(),
        output_file = f.to_string_lossy(),
        sample = sample.to_str().unwrap()
    );

    let mut p = spawn(&cmd_string, Some(20000)).unwrap();
    p.exp_regex(r#"Are you sure you want to override.*"#)
        .unwrap();
    p.send_line("y").unwrap();
    p.exp_eof().unwrap();

    let mut expected = vec![];

    File::open(&f).unwrap().read_to_end(&mut expected).unwrap();
    assert!(
        !expected.len() > 100,
        "Expected output to be printed to file"
    )
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_it_confirms_before_overwriting_a_file_and_quits() {
    let d = tempdir().unwrap();
    let f = d.as_ref().join("test.out");

    let mut file = File::create(&f).unwrap();
    file.write_all(b"I'm a file!").unwrap();

    let sample = mft_sample();

    let cmd_string = format!(
        "{bin} -f {output_file} {sample}",
        bin = cargo_bin("mft_dump").display(),
        output_file = f.to_string_lossy(),
        sample = sample.to_str().unwrap()
    );

    let mut p = spawn(&cmd_string, Some(20000)).unwrap();
    p.exp_regex(r#"Are you sure you want to override.*"#)
        .unwrap();
    p.send_line("n").unwrap();
    p.exp_eof().unwrap();

    let mut expected = vec![];

    File::open(&f).unwrap().read_to_end(&mut expected).unwrap();
    assert!(
        !expected.len() > 100,
        "Expected output to be printed to file"
    )
}
