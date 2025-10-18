// This test is designed to isolate the `sqlx::migrate!` macro.
// If this test fails to compile, it confirms the issue is with how the
// macro resolves the migrations path in an integration test context.

#[test]
fn test_migration_macro_compiles() {
    let _migrator = sqlx::migrate!();
    // The test passes if this file compiles.
    assert!(true);
}
