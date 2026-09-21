//! ## Flow
pub mod error;
pub mod slots;

/*
Key Terms

* CompliationError
    Error that occurs during compilation
* RuntimeError
    Error that occurs during runtime
* Machine?
    Compilation output, is executable with safeguards. (?)
* Slots?
    Local memory used by a machine (?)

*/

/*

Things to consider

1. Should our local memory (slots) be shared across threads?
2. How should data be access by any registered function? 
3. Should our compiled flow be executed with cocurrency in mind?

*/

#[cfg(test)]
mod tests {
    
}
