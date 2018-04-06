//CUSTOM MODULES
mod input;
mod neuron;
mod layer   ;
mod topology;
mod transfer_functions;

//USAGE
use topology::Topology;


/*

Creates a neural network and sets targets.

*/
fn main() {

    let mut T: Topology = Topology { layers: Vec::new(),targets:Vec::new(),error:1. };
    T.create_neurons([3, 2, 1].to_vec());
    T.link_neurons();
    T.targets.push(0.5);

    T.compute_outputs();
    T.print();
}
