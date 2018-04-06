use layer::Layer;
use neuron::Neuron;
use input::Input;

pub enum TRANSFER_FUNCTION_TYPES {
    LINEAR,
    BINARY,
    EXPONENTIAL,
    TANH
}
pub fn transfer_function(type_: &TRANSFER_FUNCTION_TYPES, val: f64) -> f64 {
    match type_{
        &TRANSFER_FUNCTION_TYPES::LINEAR => { transfer_function_linear(val) }
        &TRANSFER_FUNCTION_TYPES::BINARY => { transfer_function_binary(val) }
        &TRANSFER_FUNCTION_TYPES::EXPONENTIAL => { transfer_function_exp(val) }
        &TRANSFER_FUNCTION_TYPES::TANH => { transfer_function_tanh(val) }
        _=>{val }
    }
}

pub fn transfer_function_linear(val: f64) -> f64 {
    val
}

pub fn transfer_function_binary(val: f64) -> f64 {
    if val > 0.5 { 1. } else { 0. }
}

pub fn transfer_function_exp(val: f64) -> f64 {
    (-val).exp()
}
pub fn transfer_function_tanh(val: f64) -> f64 {
    let exp=(val).exp();
    let expm=(-val).exp();
    (exp-expm)/(exp+expm)
}
