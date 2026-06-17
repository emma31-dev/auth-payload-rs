# Contributing to auth-payload-rs

Thank you for your interest in contributing! Please read this document before opening a PR.

---

## General rules

- One concern per PR. Don't mix a new provider with a bug fix.
- All public items must have doc comments following the style already established in the codebase.
- Every new feature **must ship with a test** (see [Testing](#testing) below).
- Keep dependencies minimal. If a dependency is genuinely needed, justify it in the PR description.

---

## Issues

When opening an issue, include:

1. **What you expected**: the behaviour or struct shape you were looking for.
2. **What actually happened**: compiler error, wrong field type, missing field, etc.
3. **Reproduction**: a minimal code snippet or the raw JSON payload that exposed the problem.
4. **Provider and endpoint**: e.g. "Google `id_token`", "GitHub `GET /user`".
5. **Crate version**: output of `cargo pkgid auth-payload-rs`.

Bug reports without a reproduction case will be closed.

---

## Adding or expanding a provider

`auth-payload-rs` aims to cover every major OAuth2 / OIDC provider. Contributions that add a new provider or expand an existing one are very welcome.

### Adding a new provider

1. Create `src/<provider>.rs`.
2. Add a feature flag in `Cargo.toml`:
   ```toml
   [features]
   <provider> = []
   ```
3. Gate the module in `src/lib.rs`:
   ```rust
   #[cfg(feature = "<provider>")]
   pub mod <provider>;
   ```
4. Add a matching section to the crate-level docs in `src/lib.rs`.
5. Run `cargo rdme` to update the `README.md`.

### Struct requirements

- Source your field list from the **official API reference** for that provider. Link the reference in a doc comment on the struct.
- Required fields (always present in the response) → `T`.
- Optional / permission-gated / scope-gated fields → `Option<T>`.
- Use the correct primitive types (`u64` for Unix timestamps, `String` for opaque IDs, etc.).
- Include a doc comment on every field explaining what it contains and, where relevant, which scope or permission unlocks it.

### Expanding an existing provider

If a provider module is missing fields that the official API returns, open a PR that adds them as `Option<T>` (to remain backwards compatible) with a doc comment.

---

## Backend framework support

The crate currently targets Axum. Support for other frameworks is planned and contributions are welcome.

When adding support for a new backend (e.g. Salvo, Actix-web, Rocket, Poem, Tide):

1. Gate it behind a feature flag named after the framework: `salvo`, `actix`, `rocket`, etc.
2. Implement only what is necessary, typically a custom extractor or `FromRequest` impl.
3. Add a usage example to the relevant provider section in `README.md` and `src/lib.rs`.
4. Include an integration test (see [Testing](#testing)).

Do not add a framework dependency to the `default` feature set.

---

## Testing

Every PR that adds or changes behaviour **must include tests**.

### Unit tests — struct deserialisation

Place tests in a `#[cfg(test)]` module at the bottom of the relevant `src/<provider>.rs` file. Each test should deserialise a representative JSON payload and assert that fields decode to the expected values.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialises_full_payload() {
        let json = r#"{
            "id": "123456789",
            "username": "testuser",
            "discriminator": "0",
            "global_name": "Test User",
            "avatar": null,
            "bot": null,
            "system": null,
            "mfa_enabled": true,
            "banner": null,
            "accent_color": null,
            "locale": "en-US",
            "email": "user@example.com",
            "verified": true,
            "flags": 0,
            "premium_type": 2,
            "public_flags": 0
        }"#;

        let payload: DiscordIdTokenClaims = serde_json::from_str(json).unwrap();
        assert_eq!(payload.id, "123456789");
        assert_eq!(payload.username, "testuser");
        assert_eq!(payload.mfa_enabled, Some(true));
        assert_eq!(payload.email.as_deref(), Some("user@example.com"));
    }

    #[test]
    fn deserialises_minimal_payload() {
        // Only required fields — optional fields should decode as None
        let json = r#"{
            "id": "987",
            "username": "minimal",
            "discriminator": "0"
        }"#;

        let payload: DiscordIdTokenClaims = serde_json::from_str(json).unwrap();
        assert_eq!(payload.id, "987");
        assert!(payload.email.is_none());
        assert!(payload.avatar.is_none());
    }
}
```

Every provider module must have **at minimum**:

- A test with a **full payload** (all fields populated).
- A test with a **minimal payload** (only required fields; verifies `Option` fields default to `None`).

### Framework extractor tests

If you are adding a backend extractor, include an integration test that spins up a minimal test server and verifies the extractor successfully deserialises a request body.

---

## PR checklist

Before opening a PR, confirm:

- [ ] Fields sourced from the official API reference (link included in struct doc comment).
- [ ] Required fields are `T`; optional/scoped fields are `Option<T>`.
- [ ] Every public field has a doc comment.
- [ ] Full-payload and minimal-payload tests are included and pass (`cargo test --all-features`).
- [ ] `README.md` and `src/lib.rs` crate docs are updated.
- [ ] No new dependencies introduced without justification.
