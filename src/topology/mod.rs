use layer::Layer;
use neuron::Neuron;
use input::Input;
use utils::transfer_function;
use utils::random_weight;

pub struct Topology {
    pub layers: Vec<Layer>,
    pub targets: Vec<f64>,
    pub error: f64
}
/* useful functions */
impl Topology {
    /*
    A convinient get function to read the neuron neuron_nb of layer layer_nb
    */
    pub fn get(&self, layer_nb: usize, neuron_nb: usize) -> Option<&Neuron> {
        let l_opt = self.layers.get(layer_nb);
        match l_opt {
            Some(ref layer) => {
                match layer.neurons.get(neuron_nb) {
                    Some(neuron_) => {
                        Some(neuron_)
                    }
                    None => None
                }
            }
            None => None
        }
    }
    /*
    A convinient get function to get a mutable ref to the neuron neuron_nb of layer layer_nb
    */
    pub fn get_mut(&mut self, layer_nb: usize, neuron_nb: usize) -> Option<&Neuron> {
        let l_opt = self.layers.get(layer_nb);
        match l_opt {
            Some(ref layer) => {
                match layer.neurons.get(neuron_nb) {
                    Some(neuron_) => {
                        Some(neuron_)
                    }
                    None => None
                }
            }
            None => None
        }
    }
}

impl Topology {
    /*
    Creates the layers of the network and fills it with neurons and bias.
    */
    pub fn create_neurons(&mut self, T: Vec<i8>) {
        self.layers = Vec::new();
        let mut is_initial = true;
        let mut layer_id = 0;
        for t in T {
            println!("Create layer with {} neurons + bias", t);
            let mut l: Layer = Layer { id: layer_id, neurons: Vec::new(), is_initial: is_initial };
            l.init(t);
            self.layers.push(l);
            is_initial = false;
            layer_id = layer_id + 1;
        }
    }
    /*
        Creates the links between the neurons.
        In other words, sets the inputs for each layer of
         If initial layer, the single input
    */
    pub fn link_neurons(&mut self) {
        let L = &mut self.layers;
        for idx in 0..L.len() {
            //process layer l
            for n in 0..L[idx].neurons.len() {
                //process neuron n of layer l
                if idx > 0 {
                    //if layer is not initial layer, then link
                    let prev = idx - 1;
                    //create list of inputs for this neuron
                    let mut I = Vec::new();
                    for n2 in 0..L[prev].neurons.len() {
                        //add neuron previous layer
                        let id = Input { layer: prev, neuron: n2, weight: random_weight(), val: 0. };
                        I.push(id);
                    }
                    println!("Linking neuron L{}-N{} to {} neurons", idx, n, I.len());
                    L[idx].neurons[n].inputs = I;

                } else {
                    let mut I = Vec::new();
                    let id = Input { layer: 0, neuron: 0, weight: 1., val: 0.64 };
                    I.push(id);
                    println!("Linking neuron L{}-N{} to {} neurons", idx, n, I.len());
                    L[idx].neurons[n].inputs = I;
                }
            }
        }
    }
    /*
    Prints content of the layer for debug or display purposes
    */
    pub fn print(&self) {
        let L = &self.layers;
        for l in 0..L.len() {
            println!("Layer {}", l);
            L[l].print();
        }
    }
    /*

    Computes the outputs layer by layer for each neuron.

    Reads all the input of a neuron and store it in a temp vec 'outputs'
    Then, writes the output field of the neuron.
    */
    pub fn compute_outputs(&mut self) {
        println!("Compute outputs");
        for layer_id in 0..self.layers.len() {
            let mut outputs = Vec::new();//temp variable outputs to store output
            {
                let L = self.layers.get(layer_id).unwrap();
                println!("Compute layer {} outputs", L.id);
                for n in L.neurons.iter() {
                    println!("Compute L{}-N{} outputs", L.id, n.id);
                    match n.is_initial {
                        //distinguish initial layer
                        true => {
                            //initial layer neuron are just sending their input
                            let input = &n.inputs[0];//initial layer neurons are single input
                            let val = input.val;
                            println!("  L{}-N{}-I{} compute: output={} ", n.layer_id, n.id, 0, val);
                            outputs.push(val);
                        }
                        false => {
                            let mut res = 0.;
                            for (k, input) in n.inputs.iter().enumerate() {
                                //loop all inputs and sum
                                let prev = &self.layers[L.id - 1];
                                let val = prev.neurons[input.neuron].output * input.weight;
                                println!("  L{}-N{}-I{} compute: o={} *w={}=val={}", n.layer_id, n.id, k, prev.neurons[input.neuron].output, input.weight, val);
                                res = res + val;
                            }
                            outputs.push(res);//store in temp variable
                        }
                    }
                }
            }
            //write from temp variable
            {
                let mut LL = self.layers.get_mut(layer_id).unwrap();
                for (j, n) in LL.neurons.iter_mut().enumerate() {
                    n.output = transfer_function(&n.transfer_function_type, outputs[j]);
                }
            }
        }
    }

    /*
    Compute the overall error.
    Using RMS (root mean square) method
    */
    pub fn compute_errors(&mut self) {
        let mut error = 0.;
        for (idx, n) in self.layers.last().unwrap().neurons.iter().enumerate() {
            let delta = self.targets[idx] - n.output;
            error = error + delta * delta;
        }
        let N = self.layers.last().unwrap().neurons.len() as f64;
        error = error / N;
        error = error.sqrt();
        self.error = error;
    }
    pub fn set_input_values(&mut self,inputs:Vec<f64>){
        println!("set input values");
        assert!(inputs.len()==self.layers[0].neurons.len());
        for (idx,n) in self.layers[0].neurons.iter_mut().enumerate(){
            //n.inputs[0].val=inputs[idx];
            println!("set input val {}",idx);
            n.output=inputs[idx];
        }
    }
    pub fn set_targets(&mut self,targets:Vec<f64>){
        println!("set targets");
            self.targets=targets;
    }
}