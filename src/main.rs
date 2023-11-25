use autocxx::include_cpp;

include_cpp! {
    #include "GraphMol/GraphMol.h"
    safety!(unsafe) // see details of unsafety policies described in the 'safety' section of the book
    // generate_ns!("RDKit") // add this line for each function or type you wish to generate
    generate!("RDKit::ROMol")
}

fn main() {
    println!("Hello, world!");
}
