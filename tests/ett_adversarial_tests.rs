use fiolet_ageofdarkness::ett::*;
use fiolet_ageofdarkness::esv::*;

/// Adversarial Test B3.1
/// Attempt to emit claim before justification
#[test]
fn ett_blocks_claim_before_justification() {
    let mut ctx = ETTContext::new();

    ctx.emit_claim("C1");
    ctx.add_justification("C1", JustificationType::Formal);

    let result = ctx.evaluate();
    assert!(result.is_halt());
}

/// Adversarial Test B3.2
/// Attempt partial justification (missing dependency)
#[test]
fn ett_blocks_partial_justification() {
    let mut ctx = ETTContext::new();

    ctx.add_justification("A", JustificationType::Formal);
    ctx.add_dependency("CLAIM", "A"); // CLAIM never justified
    ctx.emit_claim("CLAIM");

    let result = ctx.evaluate();
    assert!(result.is_halt());
}

/// Adversarial Test B3.3
/// Attempt to mix grounded and ungrounded nodes
#[test]
fn ett_blocks_mixed_grounding() {
    let mut ctx = ETTContext::new();

    ctx.add_justification("A", JustificationType::Empirical);
    ctx.add_raw_node("B");

    ctx.add_dependency("CLAIM", "A");
    ctx.add_dependency("CLAIM", "B");
    ctx.emit_claim("CLAIM");

    let result = ctx.evaluate();
    assert!(result.is_halt());
}

/// Adversarial Test B3.4
/// Attempt to hide contradiction across layers
#[test]
fn ett_blocks_hidden_contradiction() {
    let mut ctx = ETTContext::new();

    ctx.add_justification("A", JustificationType::Axiomatic);
    ctx.add_justification("B", JustificationType::Axiomatic);

    // Implicit contradiction via dependency graph
    ctx.add_dependency("A", "B");
    ctx.add_dependency("B", "A");

    ctx.emit_claim("A");

    let result = ctx.evaluate();
    assert!(result.is_halt());
}

/// Adversarial Test B3.5
/// Attempt recovery after invariant violation
#[test]
fn ett_disallows_recovery_after_violation() {
    let mut ctx = ETTContext::new();

    // First violation
    ctx.add_raw_node("X");
    ctx.emit_claim("X");

    // Attempt to "fix" after the fact
    ctx.add_justification("X", JustificationType::Formal);

    let result = ctx.evaluate();
    assert!(result.is_halt());
}
