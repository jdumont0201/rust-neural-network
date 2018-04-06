use layer::Layer;
use neuron::Neuron;
use input::Input;
use transfer_functions::transfer_function;

pub struct Topology {
    pub layers: Vec<Layer>,
    pub targets:Vec<f64>,
    pub error:f64
}

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
        let mut L = &mut self.layers;
        for l in 0..L.len() {
            //process layer l
            for n in 0..L[l].neurons.len() {
                //process neuron n of layer l
                if l > 0 {
                    //if layer is not initial layer, then link
                    let prev = l - 1;
                    //create list of inputs for this neuron
                    let mut I = Vec::new();
                    for n2 in 0..L[prev].neurons.len() {
                        //add neuron previous layer
                        let id = Input { layer: prev, neuron: n2, weight: 1., val: 0. };
                        I.push(id);
                    }
                    L[l].neurons[n].inputs = I;
                    println!("Linking neuron L{}-N{} to {} neurons", l, n, L[l].neurons[n].inputs.len());
                } else {
                    let mut I = Vec::new();
                    let id = Input { layer: 0, neuron: 0, weight: 1., val: 0.64 };
                    I.push(id);
                    L[l].neurons[n].inputs = I;
                    println!("Linking neuron L{}-N{} to {} neurons", l, n, L[l].neurons[n].inputs.len());
                }
            }
        }
    }
    /*
    Prints content of the layer for debug or display purposes
    */
    pub fn print(&self) {
        let mut L = &self.layers;
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
                            let mut k = 0;

                            for input in n.inputs.iter() {
                                //loop all inputs and sum
                                let prev_opt = self.layers.get(L.id - 1);
                                let val;
                                match prev_opt {
                                    Some(prev) => {
                                        val = prev.neurons[input.neuron].output * input.weight;
                                        println!("  L{}-N{}-I{} compute: o={} *w={}=val={}", n.layer_id, n.id, k, prev.neurons[input.neuron].output, input.weight, val);
                                    }
                                    None => {
                                        val = 0.;
                                        println!("!! accessing prev layer failed")
                                    }
                                }
                                k = k + 1;
                                res = res + val;
                            }
                            outputs.push(res);//store in temp variable
                        }
                    }
                }
            }
            //write from temp variable
            {
                let mut j = 0;
                let mut LL = self.layers.get_mut(layer_id).unwrap();
                for n in LL.neurons.iter_mut() {
                    n.output = transfer_function(&n.transfer_function_type, outputs[j]);


                    j + j + 1;
                }
            }
        }
    }
    pub fn compute_errors(&mut self){
        let mut error=0.;
        for (idx,n) in self.layers.last().unwrap().neurons.iter().enumerate(){
            let delta=self.targets[idx]-n.output;
            error=error+delta*delta;
        }
        let N=self.layers.last().unwrap().neurons.len() as f64;
        error=error/N;
        error=error.sqrt();
        self.error=error;
    }
}