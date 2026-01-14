use fiolet_ageofdarkness::ett::*;

/// B4 Meta-Test
/// Any failure must result in HARD HALT
#[test]
fn ett_all_failures_map_to_hard_halt() {
    let mut ctx = ETTContext::new();

    // Deliberate multi-failure state
    ctx.add_raw_node("X");
    ctx.add_dependency("X", "X");
    ctx.emit_claim("X");

    let result = ctx.evaluate();

    assert!(result.is_halt());
    assert!(result.is_terminal());
}
