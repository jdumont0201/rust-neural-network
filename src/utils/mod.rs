use rand::{Rng, thread_rng};
pub enum TransferFunctionTypes {
    LINEAR,
    BINARY,
    EXPONENTIAL,
    TANH
}
pub fn transfer_function(type_: &TransferFunctionTypes, val: f64) -> f64 {
    match type_{
        &TransferFunctionTypes::LINEAR => { transfer_function_linear(val) }
        &TransferFunctionTypes::BINARY => { transfer_function_binary(val) }
        &TransferFunctionTypes::EXPONENTIAL => { transfer_function_exp(val) }
        &TransferFunctionTypes::TANH => { transfer_function_tanh(val) }
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

pub fn random_weight() -> f64{
    let mut rng = thread_rng();
    rng.gen()
}