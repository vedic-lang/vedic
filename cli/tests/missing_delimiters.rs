use std::fs;
use std::process::Command;

#[test]
fn missing_delimiter_marks_the_insertion_point_on_stderr() {
    let path = std::env::temp_dir().join(format!("vedic-delimiter-{}.ved", std::process::id()));
    for (source, location, line, marker) in [
        (
            "# start\r\n\tवद(\"दुर्गम\"; # note\r\n",
            "line 2, column 13",
            "\tवद(\"दुर्गम\"; # note",
            "\t         ^ insert ')' here",
        ),
        (
            "मान नाम = \"पहली\r\nदुर्गम\"\r\n# end\r\n",
            "line 2, column 8",
            "दुर्गम\"",
            "     ^ insert ';' here",
        ),
        ("वद(;", "line 1, column 4", "वद(;", "   ^ insert ')' here"),
        (
            "सूत्र f( {}",
            "line 1, column 9",
            "सूत्र f( {}",
            "      ^ insert ')' here",
        ),
        (
            "सूत्र f(){ फल }",
            "line 1, column 14",
            "सूत्र f(){ फल }",
            "           ^ insert ';' here",
        ),
    ] {
        fs::write(&path, source).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_vedic"))
            .arg(&path)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(65));
        assert!(output.stdout.is_empty());
        let error = String::from_utf8(output.stderr).unwrap();
        assert_eq!(error.matches("सङ्कलन-काले-दोषः").count(), 1, "{error}");
        assert!(
            error.contains(&format!("File \"{}\", {location}\n", path.display())),
            "{error}"
        );
        assert!(
            error.ends_with(&format!("    {line}\n    {marker}\n")),
            "{error}"
        );
    }
    fs::remove_file(path).unwrap();
}
