extern crate rand;

//CUSTOM MODULES
mod input;
mod neuron;
mod layer   ;
mod topology;
mod utils;

//USAGE
use topology::Topology;


/*

Creates a neural network and propagate.

*/
fn main() {
    let input_values=[0.2,0.3,0.4,1.].to_vec();
    let targets=[0.2,0.3].to_vec();

    //create network: neurons & links
    let mut T: Topology = Topology { layers: Vec::new(),targets:Vec::new(),error:1. };
    T.create_neurons([3, 3, 2].to_vec());
    T.link_neurons();

    //set inputs values and targets
    T.set_input_values(input_values);
    T.set_targets(targets);

    //update outputs
    T.compute_outputs();

    //display
    T.print();
}
