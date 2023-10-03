# Rust Principles

**Box deallocation principle:**
If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.

**Moved heap data principle:** 
if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.

**Pointer Safety Principle:**
Pointer Safety Principle: data should never be aliased and mutated at the same time.

