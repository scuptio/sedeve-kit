------------------------------ MODULE StateDB ------------------------------

LOCAL INSTANCE TLC
LOCAL INSTANCE Integers
  (*************************************************************************)
  (* Imports the definitions from the modules, but doesn't export them.    *)
  (*************************************************************************)


SaveValue(state, path) ==
  (*************************************************************************)
  (* NewState would store state to a give database if it not exist         *)
  (*************************************************************************)
  TRUE

QueryAllValues(path) ==
  (*************************************************************************)
  (* QueryAllStates would load a set of state from the given database.     *)
  (*************************************************************************)
  CHOOSE val : TRUE

FlushAll ==
  TRUE



============================================================================
