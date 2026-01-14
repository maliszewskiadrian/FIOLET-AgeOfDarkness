use fiolet_ageofdarkness::ett::*;
use fiolet_ageofdarkness::esv::*;

/// Boundary Test B2.1
/// Infinite trace must HARD HALT
#[test]
fn ett_rejects_infinite_trace() {
    let mut ctx = ETTContext::new();

    // Artificially create a cyclic justification
    ctx.add_justification("A", JustificationType::Axiomatic);
    ctx.add_dependency("A", "A"); // self-cycle

    let result = ctx.evaluate();

    assert!(result.is_halt());
}

/// Boundary Test B2.2
/// Orphan claim must HARD HALT
#[test]
fn ett_rejects_orphan_claim() {
    let mut ctx = ETTContext::new();

    // Claim without justification
    ctx.emit_claim("Unjustified claim");

    let result = ctx.evaluate();

    assert!(result.is_halt());
}

/// Boundary Test B2.3
/// Contradictory trace must HARD HALT
#[test]
fn ett_rejects_reflexive_contradiction() {
    let mut ctx = ETTContext::new();

    ctx.add_justification("A", JustificationType::Axiomatic);
    ctx.add_justification("NOT_A", JustificationType::Axiomatic);

    ctx.add_dependency("A", "NOT_A"); // contradiction
    ctx.emit_claim("A");

    let result = ctx.evaluate();

    assert!(result.is_halt());
}

/// Boundary Test B2.4
/// Ungrounded node must HARD HALT
#[test]
fn ett_rejects_ungrounded_node() {
    let mut ctx = ETTContext::new();

    // Node without classification
    ctx.add_raw_node("X");
    ctx.emit_claim("X");

    let result = ctx.evaluate();

    assert!(result.is_halt());
}

/// Boundary Test B2.5
/// Multiple invariant violations must still HALT
#[test]
fn ett_rejects_multi_violation_state() {
    let mut ctx = ETTContext::new();

    // Orphan + ungrounded + cyclic
    ctx.add_raw_node("Z");
    ctx.add_dependency("Z", "Z");
    ctx.emit_claim("Z");

    let result = ctx.evaluate();

    assert!(result.is_halt());
}
