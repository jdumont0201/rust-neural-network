use neuron::Neuron;
//use topology::Topology;
use utils::TransferFunctionTypes;

pub struct Layer {
    pub id:usize,
    pub neurons: Vec<Neuron>,
    pub is_initial: bool,
}
impl Layer {
    pub fn init(&mut self,  n: i8) {
        self.neurons = Vec::new();
        let is_initial = self.id == 0;
        for i in 0..n {
            println!("  L{} Add neuron",self.id);
            self.neurons.push(Neuron { layer_id:self.id,id:i,is_bias: false, inputs: Vec::new(), is_initial: is_initial, output: 0., threshold: 0., transfer_function_type:TransferFunctionTypes::LINEAR})
        }
        println!("  L{} Add bias",self.id);
        self.neurons.push(Neuron { id:n,layer_id:self.id,is_bias: true, inputs: Vec::new(), is_initial: is_initial, output: 0., threshold: 0.,transfer_function_type:TransferFunctionTypes::LINEAR })
    }
    pub fn compute_outputs(&mut self,prev:&Layer) {
        println!("  L{}  Compute outputs",self.id);
        for n in 0..self.neurons.len() {
            self.neurons[n].compute_output(prev);
        }
    }
    pub fn print(&self){
        for n in 0..self.neurons.len() {
            self.neurons[n].print();
        }
    }
}