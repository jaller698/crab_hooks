use rusty_hooker::hook_types::HookTypes;
use std::str::FromStr;

#[test]
fn test_display_and_parse_all_variants() {
    // Mapping of enum variants to their kebab-case string representations
    let variants = vec![
        (HookTypes::PreCommit, "pre-commit"),
        (HookTypes::PrepareCommitMsg, "prepare-commit-msg"),
        (HookTypes::CommitMsg, "commit-msg"),
        (HookTypes::PostCommit, "post-commit"),
        (HookTypes::ApplyPatchMsg, "apply-patch-msg"),
        (HookTypes::PreApplyPatch, "pre-apply-patch"),
        (HookTypes::PostApplyPatch, "post-apply-patch"),
        (HookTypes::PreRebase, "pre-rebase"),
        (HookTypes::PostRewrite, "post-rewrite"),
        (HookTypes::PostCheckout, "post-checkout"),
        (HookTypes::PostMerge, "post-merge"),
        (HookTypes::PrePush, "pre-push"),
        (HookTypes::PreAutoGC, "pre-auto-gc"),
        (HookTypes::PreReceive, "pre-receive"),
        (HookTypes::Update, "update"),
        (HookTypes::PostReceive, "post-receive"),
    ];

    for (variant, expected_str) in variants {
        // Check Display implementation
        let display_str = variant.to_string();
        assert_eq!(
            display_str, expected_str,
            "Display for {:?} should be {}",
            variant, expected_str
        );

        // Check FromStr parsing
        let parsed = HookTypes::from_str(expected_str).expect("Parsing should succeed");
        assert_eq!(
            parsed, variant,
            "Parsing {} should yield {:?}",
            expected_str, variant
        );
    }
}

#[test]
fn test_parse_invalid_string() {
    // An invalid string should return an Err
    let invalid = "non-existent-hook";
    let parsed = HookTypes::from_str(invalid);
    assert!(parsed.is_err(), "Parsing invalid string should error");
}
