use crate::common::util::*;

static INPUT: &str = "sorted.txt";
static OUTPUT: &str = "sorted-output.txt";
static SKIP_CHARS: &str = "skip-chars.txt";
static SKIP_FIELDS: &str = "skip-fields.txt";
static SORTED_ZERO_TERMINATED: &str = "sorted-zero-terminated.txt";

#[test]
fn test_stdin_default() {
    new_ucmd!()
        .pipe_in_fixture(INPUT)
        .run()
        .stdout_is_fixture("sorted-simple.expected");
}

#[test]
fn test_single_default() {
    new_ucmd!()
        .arg(INPUT)
        .run()
        .stdout_is_fixture("sorted-simple.expected");
}

#[test]
fn test_single_default_output() {
    let (at, mut ucmd) = at_and_ucmd!();
    let expected = at.read("sorted-simple.expected");
    ucmd.args(&[INPUT, OUTPUT]).run();
    let found = at.read(OUTPUT);
    assert_eq!(found, expected);
}

#[test]
fn test_stdin_counts() {
    new_ucmd!()
        .args(&["-c"])
        .pipe_in_fixture(INPUT)
        .run()
        .stdout_is_fixture("sorted-counts.expected");
}

#[test]
fn test_stdin_skip_1_char() {
    new_ucmd!()
        .args(&["-s1"])
        .pipe_in_fixture(SKIP_CHARS)
        .run()
        .stdout_is_fixture("skip-1-char.expected");
}

#[test]
fn test_stdin_skip_5_chars() {
    new_ucmd!()
        .args(&["-s5"])
        .pipe_in_fixture(SKIP_CHARS)
        .run()
        .stdout_is_fixture("skip-5-chars.expected");
}

#[test]
fn test_stdin_skip_and_check_2_chars() {
    new_ucmd!()
        .args(&["-s3", "-w2"])
        .pipe_in_fixture(SKIP_CHARS)
        .run()
        .stdout_is_fixture("skip-3-check-2-chars.expected");
}

#[test]
fn test_stdin_skip_1_field() {
    new_ucmd!()
        .args(&["-f2"])
        .pipe_in_fixture(SKIP_FIELDS)
        .run()
        .stdout_is_fixture("skip-2-fields.expected");
}

#[test]
fn test_stdin_all_repeated() {
    new_ucmd!()
        .args(&["--all-repeated"])
        .pipe_in_fixture(INPUT)
        .run()
        .stdout_is_fixture("sorted-all-repeated.expected");
}

#[test]
fn test_stdin_all_repeated_separate() {
    new_ucmd!()
        .args(&["--all-repeated=separate"])
        .pipe_in_fixture(INPUT)
        .run()
        .stdout_is_fixture("sorted-all-repeated-separate.expected");
}

#[test]
fn test_stdin_all_repeated_prepend() {
    new_ucmd!()
        .args(&["--all-repeated=prepend"])
        .pipe_in_fixture(INPUT)
        .run()
        .stdout_is_fixture("sorted-all-repeated-prepend.expected");
}

#[test]
fn test_stdin_unique_only() {
    new_ucmd!()
        .args(&["-u"])
        .pipe_in_fixture(INPUT)
        .run()
        .stdout_is_fixture("sorted-unique-only.expected");
}

#[test]
fn test_stdin_repeated_only() {
    new_ucmd!()
        .args(&["-d"])
        .pipe_in_fixture(INPUT)
        .run()
        .stdout_is_fixture("sorted-repeated-only.expected");
}

#[test]
fn test_stdin_ignore_case() {
    new_ucmd!()
        .args(&["-i"])
        .pipe_in_fixture(INPUT)
        .run()
        .stdout_is_fixture("sorted-ignore-case.expected");
}

#[test]
fn test_stdin_zero_terminated() {
    new_ucmd!()
        .args(&["-z"])
        .pipe_in_fixture(SORTED_ZERO_TERMINATED)
        .run()
        .stdout_is_fixture("sorted-zero-terminated.expected");
}

#[test]
fn test_invalid_utf8() {
    new_ucmd!()
        .arg("not-utf8-sequence.txt")
        .run()
        .failure()
        .stderr_only(
            "uniq: failed to convert line to utf8: invalid utf-8 sequence of 1 bytes from index 0",
        );
}

#[test]
fn test_group() {
    new_ucmd!()
        .args(&["--group"])
        .pipe_in_fixture(INPUT)
        .run()
        .stdout_is_fixture("group.expected");
}

#[test]
fn test_group_prepend() {
    new_ucmd!()
        .args(&["--group=prepend"])
        .pipe_in_fixture(INPUT)
        .run()
        .stdout_is_fixture("group-prepend.expected");
}

#[test]
fn test_group_append() {
    new_ucmd!()
        .args(&["--group=append"])
        .pipe_in_fixture(INPUT)
        .run()
        .stdout_is_fixture("group-append.expected");
}

#[test]
fn test_group_both() {
    new_ucmd!()
        .args(&["--group=both"])
        .pipe_in_fixture(INPUT)
        .run()
        .stdout_is_fixture("group-both.expected");
}

#[test]
fn test_group_separate() {
    new_ucmd!()
        .args(&["--group=separate"])
        .pipe_in_fixture(INPUT)
        .run()
        .stdout_is_fixture("group.expected");
}
