#![cfg(test)]

mod sep24_origin_pinning_tests {
    use anchorkit::sep24::{
        validate_interactive_url, initiate_interactive_deposit_with_origin,
        initiate_interactive_withdrawal_with_origin,
        RawInteractiveDepositResponse, RawInteractiveWithdrawalResponse,
    };

    // ── validate_interactive_url origin pinning ───────────────────────────

    #[test]
    fn url_matching_origin_accepted() {
        assert!(validate_interactive_url(
            "https://anchor.example.com/sep24/deposit?asset=USDC",
            Some("https://anchor.example.com"),
        )
        .is_ok());
    }

    #[test]
    fn url_on_different_domain_rejected() {
        assert!(validate_interactive_url(
            "https://evil.example.com/sep24/deposit",
            Some("https://anchor.example.com"),
        )
        .is_err());
    }

    #[test]
    fn url_same_domain_different_port_rejected() {
        assert!(validate_interactive_url(
            "https://anchor.example.com:8443/sep24/deposit",
            Some("https://anchor.example.com"),
        )
        .is_err());
    }

    #[test]
    fn allowed_origin_none_skips_check_accepts_valid_https() {
        assert!(validate_interactive_url(
            "https://any-anchor.example.org/sep24/deposit",
            None,
        )
        .is_ok());
    }

    #[test]
    fn origin_comparison_is_case_insensitive() {
        // Both URL and allowed_origin use the same host, just different casing
        // in the origin string (the URL itself must be valid https://).
        assert!(validate_interactive_url(
            "https://Anchor.Example.COM/sep24/deposit",
            Some("https://anchor.example.com"),
        )
        .is_ok());
    }

    #[test]
    fn encoded_mixed_case_redirect_parameter_is_rejected() {
        assert!(validate_interactive_url(
            "https://anchor.example.com/sep24/deposit?%52%45%54%55%52%4E_%55%52%4C=https%3A%2F%2Fevil.example.com",
            None,
        )
        .is_err());
    }

    #[test]
    fn invalid_allowed_origin_is_rejected() {
        assert!(validate_interactive_url(
            "https://anchor.example.com/sep24/deposit",
            Some("not a valid origin"),
        )
        .is_err());
    }

    // ── initiate_interactive_deposit_with_origin ─────────────────────────

    #[test]
    fn deposit_with_matching_origin_succeeds() {
        let raw = RawInteractiveDepositResponse {
            url: "https://anchor.example.com/deposit".into(),
            id: "tx-001".into(),
        };
        assert!(
            initiate_interactive_deposit_with_origin(raw, Some("https://anchor.example.com"))
                .is_ok()
        );
    }

    #[test]
    fn deposit_with_mismatched_origin_rejected() {
        let raw = RawInteractiveDepositResponse {
            url: "https://evil.example.com/deposit".into(),
            id: "tx-001".into(),
        };
        assert!(
            initiate_interactive_deposit_with_origin(raw, Some("https://anchor.example.com"))
                .is_err()
        );
    }

    // ── initiate_interactive_withdrawal_with_origin ───────────────────────

    #[test]
    fn withdrawal_with_matching_origin_succeeds() {
        let raw = RawInteractiveWithdrawalResponse {
            url: "https://anchor.example.com/withdraw".into(),
            id: "tx-002".into(),
        };
        assert!(
            initiate_interactive_withdrawal_with_origin(raw, Some("https://anchor.example.com"))
                .is_ok()
        );
    }

    #[test]
    fn withdrawal_with_mismatched_origin_rejected() {
        let raw = RawInteractiveWithdrawalResponse {
            url: "https://other.example.com/withdraw".into(),
            id: "tx-002".into(),
        };
        assert!(
            initiate_interactive_withdrawal_with_origin(raw, Some("https://anchor.example.com"))
                .is_err()
        );
    }

    // ── #836 non-HTTPS (cleartext) redirect URLs are rejected ─────────────

    #[test]
    fn validate_interactive_url_rejects_http_scheme() {
        assert!(validate_interactive_url("http://anchor.example.com/deposit", None).is_err());
        assert!(validate_interactive_url("https://anchor.example.com/deposit", None).is_ok());
    }

    #[test]
    fn deposit_with_http_redirect_url_rejected() {
        let raw = RawInteractiveDepositResponse {
            url: "http://anchor.example.com/deposit".into(),
            id: "tx-001".into(),
        };
        assert!(
            initiate_interactive_deposit_with_origin(raw, Some("https://anchor.example.com"))
                .is_err()
        );
    }

    #[test]
    fn withdrawal_with_http_redirect_url_rejected() {
        let raw = RawInteractiveWithdrawalResponse {
            url: "http://anchor.example.com/withdraw".into(),
            id: "tx-002".into(),
        };
        assert!(
            initiate_interactive_withdrawal_with_origin(raw, Some("https://anchor.example.com"))
                .is_err()
        );
    }

    #[test]
    fn https_redirect_url_accepted_after_scheme_check() {
        let raw = RawInteractiveDepositResponse {
            url: "https://anchor.example.com/deposit".into(),
            id: "tx-003".into(),
        };
        let out = initiate_interactive_deposit_with_origin(
            raw,
            Some("https://anchor.example.com"),
        )
        .expect("https redirect must pass");
        assert_eq!(out.url, "https://anchor.example.com/deposit");
    }
}
