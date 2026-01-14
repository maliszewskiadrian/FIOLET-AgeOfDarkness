from fiolet_py.ett import KnowledgeClass, ETTState, ett_trigger

def test_allows_grounded():
    assert ett_trigger(KnowledgeClass.GROUNDED) == ETTState.ALLOW

def test_halts_ungrounded():
    assert ett_trigger(KnowledgeClass.UNGROUNDED) == ETTState.HALT

def test_halts_contradictory():
    assert ett_trigger(KnowledgeClass.CONTRADICTORY) == ETTState.HALT
