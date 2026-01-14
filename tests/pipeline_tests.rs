use fiolet::pipeline::engine::{run_pipeline, PipelineResult};

#[test]
fn pipeline_allows_valid_knowledge() {
    let result = run_pipeline(true, false);
    assert_eq!(result, PipelineResult::Output);
}

#[test]
fn pipeline_halts_without_source() {
    let result = run_pipeline(false, false);
    assert_eq!(result, PipelineResult::Halt);
}

#[test]
fn pipeline_halts_on_contradiction() {
    let result = run_pipeline(true, true);
    assert_eq!(result, PipelineResult::Halt);
}
