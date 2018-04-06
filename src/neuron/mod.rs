use input::Input;
//use topology::Topology;
use layer::Layer;
use utils::TransferFunctionTypes;
pub struct Neuron {
    pub id:i8,
    pub layer_id:usize,
    pub inputs: Vec<Input>,
    pub output: f64,
    pub threshold: f64,
    pub is_initial: bool,    //single input
    pub is_bias: bool,//no input
    pub transfer_function_type:TransferFunctionTypes

}

impl Neuron {

    pub fn compute_output(&mut self,L:& Layer) {
        println!("  L{}-N{}  Compute outputs",self.layer_id, self.id);
        match self.is_initial {
            true => {
                //initial layer neuron are just sending their input
                let input = &self.inputs[0];//initial layer neurons are single input
                println!("  L{}-N{}-I{} compute {} ",self.layer_id,self.id,0,input.val);
                self.output = input.val;//T.get(input.layer,input.neuron).in
            }
            false => {
                let mut res = 0.;
                for i in 0..self.inputs.len() {
                    let input = &self.inputs[i];
                    let val=L.neurons[input.neuron].output;
                    println!("  L{}-N{}-I{} compute {} {}",self.layer_id,self.id,i,val,input.weight);
                    res = res + input.val * input.weight;
                }
                self.output = res;
            }
        }
    }
    pub fn print(&self){
        let n=&self.id;
        for i in 0..self.inputs.len() {
            println!("  L{}-N{}-I{} val={} weight={}", self.layer_id,n,i, &self.inputs[i].val, &self.inputs[i].weight)
        }

        println!("    L{}-N{} output={}", self.layer_id,n, &self.output)

    }
}
