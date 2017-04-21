# CComplex-rs
A easy to use library for complex numbers.

CComplex makes complex calculations easy to type.  
```rust
extern crate ccomplex;
use ccomplex::iprimitive::I;

let z = 5.0 + I(4.0) / 5.0 - I(6.0) * 4.0;
println!("{}", z);
```
Prints: `5-23.2i`