use connection::Connection;
use topology::Topology;
use layer::Layer;
use utils::TransferFunctionTypes;

pub struct Neuron {
    pub id: i8,
    pub layer_id: usize,
    pub connections: Vec<Connection>,
    pub output: f64,
    pub is_initial: bool,
    //single input
    pub is_bias: bool,
    //no input
    pub transfer_function_type: TransferFunctionTypes,
    pub gradient: f64
}

impl Neuron {
    pub fn compute_output(&mut self, L: &Layer) {
        println!("  L{}-N{}  Compute outputs", self.layer_id, self.id);
        match self.is_initial {
            true => {
                //initial layer neuron are just sending their input
                let input = &self.connections[0];//initial layer neurons are single input
                println!("  L{}-N{}-I{} compute {} ", self.layer_id, self.id, 0, input.val);
                self.output = input.val;//T.get(input.layer,input.neuron).in
            }
            false => {
                let mut res = 0.;
                for i in 0..self.connections.len() {
                    let input = &self.connections[i];
                    let val = L.neurons[input.neuron].output;
                    println!("  L{}-N{}-I{} compute {} {}", self.layer_id, self.id, i, val, input.weight);
                    res = res + input.val * input.weight;
                }
                self.output = res;
            }
        }
    }
    pub fn print(&self) {
        let n = &self.id;
        for i in 0..self.connections.len() {
            println!("  L{}-N{}-I{} val={} weight={}", self.layer_id, n, i, &self.connections[i].val, &self.connections[i].weight)
        }

        println!("    L{}-N{} output={}", self.layer_id, n, &self.output)
    }
    pub fn sum_of_DOW(&self, T: &Topology) -> f64 {
        //assert not last layer
        assert!(self.layer_id != T.layers.len());

        let mut sum = 0.;
        //browse next layer neurons
        for (idx, n) in T.layers[self.layer_id + 1].neurons.iter().enumerate() {
            //for each connection, if is current neuron
            for (i_idx, input) in n.connections.iter().enumerate() {
                if self.id == i_idx as i8{
                    //add connection weight * neuron gradient
                    sum = sum + input.weight * T.layers[input.layer].neurons[input.neuron].gradient;
                }
            }

        }
        sum
    }
}
