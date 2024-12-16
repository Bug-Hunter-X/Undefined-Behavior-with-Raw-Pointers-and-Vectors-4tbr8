# Undefined Behavior with Raw Pointers and Vectors in Rust

This example demonstrates a common mistake in Rust when working with raw pointers and vectors.  Modifying a vector through a raw pointer after the vector's capacity might have changed leads to undefined behavior. The solution demonstrates safer alternatives using indexing or borrowing.

**Bug:** The original code uses `as_mut_ptr()` to obtain a raw pointer to the vector's data.  Subsequent modification via this raw pointer can cause memory corruption or other unexpected results if the vector's internal representation is modified (e.g., reallocation due to pushing more elements).

**Solution:** The solution showcases safer ways to modify vector elements: using indexing or by borrowing.