use dfdx::{tensor::{Tensor, Cpu},shapes::Rank1};



pub struct Observation {
    pub view: Tensor<Rank1<8>,f32,Cpu>
}