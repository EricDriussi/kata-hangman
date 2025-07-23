use hangman::failures::AllowedFailures;

#[test]
fn remaining_failures_are_consumed() {
    let limit = 3;
    let mut failures = AllowedFailures::limit(limit);

    failures.consume();

    assert_eq!(failures.remaining(), limit - 1);
}

#[test]
fn some_failures_left() {
    let mut failures = AllowedFailures::limit(2);

    failures.consume();

    assert!(!failures.none_left());
}

#[test]
fn no_failures_left() {
    let mut failures = AllowedFailures::limit(1);

    failures.consume();

    assert!(failures.none_left());
}
