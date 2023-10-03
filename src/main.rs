use autocxx::include_cpp;

include_cpp! {
    #include "RDGeneral/RDAny.h"
    #include "GraphMol/ROMol.h"
    safety!(unsafe) // see details of unsafety policies described in the 'safety' section of the book
    generate_ns!("RDKit") // add this line for each function or type you wish to generate
}

fn main() {
    println!("Hello, world!");
}
