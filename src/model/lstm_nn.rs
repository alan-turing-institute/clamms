use tch::{nn, Device, Tensor, Kind};
use tch::nn::{OptimizerConfig, Module, RNN};

// Output sizes
const POLICY_OUT: i64 = 3; // Policy head - to food, to water, stationary
const REWARD_OUT: i64 = 1; // Single value reward

// Input size - (food, water) ([n, 2]) + last action ([n, 3])
const INPUT_SIZE: i64 = 5;

// LSTM size
const LSTM_SIZE: i64 = 128;

// Learning rate
const LEARNING_RATE: f64 = 0.01;

pub fn forward_pass(x1: Tensor, x2: Tensor, y: Tensor) -> (Tensor, Tensor) {

    // x1 = [food, water]
    // x2 = One-hot encoded action e.g. [0, 1, 0]

    // Currently running on CPU so let's stick to that
    let device = Device::Cpu;
    // let device = Device::cuda_if_available();

    // Create new VarStore
    // Not quite sure what a variable store is
    // Device is going to be either CPU or GPU
    let vs = nn::VarStore::new(device);

    // Concat inputs 
    // Eventually visual info can be concatenated here as well
    // Not sure this is the best way to do it
    let x = Tensor::concatenate(&[x1, x2], 2);

    // LSTM layer initialisation
    // Varstore, input size, number of hidden units, config
    let lstm = nn::lstm(vs.root(), INPUT_SIZE, LSTM_SIZE, Default::default());

    // MLP
    let basic_linear = nn::linear(vs.root(), LSTM_SIZE, 64, Default::default());

    // Policy head
    let linear_policy = nn::linear(vs.root(), 64, POLICY_OUT, Default::default());

    // Value head
    let linear_reward = nn::linear(vs.root(), 64, REWARD_OUT, Default::default());

    // Set up Adam optimizer - not necessary for forward pass
    //let mut opt = nn::Adam::default().build(&vs, LEARNING_RATE);

    // Forward pass
    let (lstm_out, _) = lstm.seq(&x.to_device(device));
    let l1 = basic_linear.forward(&lstm_out).relu(); 
    let l2 = basic_linear.forward(&lstm_out).relu();
    let policy_out = linear_policy.forward(&l1).softmax(-1, Kind::Float);
    let reward_out = linear_reward.forward(&l2);

    return(policy_out, reward_out)  
}

#[cfg(test)]
mod tests {
    use crate::model::{utils::encode_batch, 
        action::Action, 
        agent_state::{AgentState, encode_vec_of_states}, 
        reward::Reward};

    use super::*;

    #[test]
    fn test_fp_output() { // Currently fails due to x1 and x2 not being the same size
        
        // Create Action
        let enc_action = encode_batch(&[Action::ToFood.encode()]);

        // Create AgentState
        let v = vec![AgentState {food: 25, water: 7}];
        let a = encode_vec_of_states(&v);
        let enc_agent_state = encode_batch(&[a]);

        // Create Reward
        let r = Reward::new(52);
        let enc_reward = encode_batch(&[r.encode()]);

        let (policy, reward) = forward_pass(enc_action, enc_agent_state, enc_reward);

    }
     
}
