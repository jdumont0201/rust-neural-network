//CUSTOM MODULES
mod input;
mod neuron;
mod layer   ;
mod topology;
mod transfer_functions;

//USAGE
use topology::Topology;


fn main() {
    println!("Rust Neural Networks");
    let mut T: Topology = Topology { layers: Vec::new() };
    T.create_neurons([3, 2, 1].to_vec());
    T.link_neurons();

    T.compute_outputs();
    T.print();
}
