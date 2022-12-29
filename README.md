# rsScrabble

This repository is meant to be a submodule from the repository twiby/pySrabble. Please check pyScrabble directly for more information.

This crate is written in Rust, using pyo3 for python binding, and it provides a few primitives to compute the best play in a scrabble game. 
The file src/lib.rs contains a lightweight API available to Python code (it is 2 classes, one of which has 4 functions and a contructor).
