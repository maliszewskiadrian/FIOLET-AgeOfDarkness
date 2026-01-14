from enum import Enum, auto

class KnowledgeClass(Enum):
    GROUNDED = auto()
    UNGROUNDED = auto()
    CONTRADICTORY = auto()

class ETTState(Enum):
    ALLOW = auto()
    HALT = auto()

def ett_trigger(knowledge: KnowledgeClass) -> ETTState:
    """
    Deterministic Epistemic Termination Trigger.
    Fail-closed by default.
    """
    if knowledge == KnowledgeClass.GROUNDED:
        return ETTState.ALLOW
    return ETTState.HALT
