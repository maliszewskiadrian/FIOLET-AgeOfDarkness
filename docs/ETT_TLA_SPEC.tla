---- MODULE ETT ----
EXTENDS Naturals

(*
ETT – Epistemic Termination Trigger
Formal TLA+ specification
*)

CONSTANTS Grounded, Ungrounded, Contradictory
CONSTANTS Allow, Halt

VARIABLES knowledge, ettState

(* Initial state *)
Init ==
    knowledge \in {Grounded, Ungrounded, Contradictory}
    /\ ettState \in {Allow, Halt}

(* Transition function *)
Next ==
    /\ knowledge' = knowledge
    /\ ettState' =
        IF knowledge = Grounded
        THEN Allow
        ELSE Halt

(* Safety invariants *)
Inv1 == (knowledge = Contradictory) => (ettState = Halt)
Inv2 == (knowledge = Ungrounded) => (ettState = Halt)
Inv3 == (knowledge = Grounded) => (ettState = Allow)

Invariant ==
    Inv1 /\ Inv2 /\ Inv3

Spec ==
    Init /\ [][Next]_<<knowledge, ettState>>

THEOREM Spec => []Invariant
==== 
