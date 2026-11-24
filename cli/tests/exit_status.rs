use std::process::Command;

#[test]
fn invalid_invocations_report_the_argument_and_fail() {
    for (argument, expected_status) in [
        ("--unknown-option", 64),
        ("नमस्ते.txt", 64),
        ("missing-script.ved", 66),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_vedic"))
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .arg(argument)
            .output()
            .unwrap();

        assert_eq!(output.status.code(), Some(expected_status), "{argument}");
        assert!(output.stdout.is_empty(), "{argument}");
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains(argument), "{stderr}");
    }
}

#[test]
fn running_a_script_preserves_interpreter_exit_codes() {
    for (script, expected_status) in [
        ("../tests/block/empty.ved", 0),
        ("../tests/mitra/mitra_without_dot.ved", 65),
        ("../tests/maan/undefined_global.ved", 70),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_vedic"))
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .arg(script)
            .output()
            .unwrap();

        assert_eq!(output.status.code(), Some(expected_status), "{script}");
    }
}
