#[allow(unused_imports)]

// goal 1. rewrite (bevy)
// goal 1.1 rewrite (w/ gym-rs) 
// goal 2. modularize
// goal 3. burn backend
// goal 4. candle backend

use rand::prelude::*;

use bevy::{prelude::*, sprite, window::PresentMode};

use dfdx::losses::mse_loss;
use dfdx::prelude::{Linear, ReLU};
use dfdx::nn::{Module, ResetParams};
use dfdx::optim::{Adam, Optimizer};
use dfdx::tensor::{Cpu, Tensor1D, Tensor2D};
// dropped: HasArrayData, TensorCreator

use dfdx::tensor_ops::SelectTo; 
// dropped: 
// use dfdx::tensor_ops::Select1 
//   unsure if this is a correct replacement. 


const NEXT_STATE_DISCOUNT: f32 = 0.9; // ?
const BATCH_SIZE: usize = 64;
const EPSILON_DECAY: f32 = 0.0002; // ? quickness reward? 

// Cart pole variables from OpenAI
const GRAVITY: f32 = 9.81;
const MASS_CART: f32 = 1.;
const MASS_POLE: f32 = 0.1;
const TOTAL_MASS: f32 = MASS_POLE * MASS_CART; // why is this multiplied... not added? 
const LENGTH: f32 = 0.5;
const POLEMASS_LENGTH: f32 = MASS_POLE * MASS_CART; // ?
const FORCE_MAG: f32 = 10.; // ? force control? 
const TAU: f32 = 0.002; // time-step (discrete size)
const THETA_THRESHOLD_RADIANS: f32 = 12. * 2. * std::f32::consts::PI / 360.; // failure threshold ? not sure
const X_THRESHOLD: f32 = 2.4;

const ARENA_WIDTH: f32 = 2. * X_THRESHOLD;
const ARENA_HEIGHT: f32 = ARENA_WIDTH * (9. / 16.);

type Mlp = (
    Linear<4, 64>,
    (Linear<64, 64>, ReLU),
    (Linear<64, 32>, ReLU),
    Linear<32, 2>,
);

type Transition = ([f32; 4], i32, i32, Option<[f32; 4]>); // ? 

#[derive(Debug)] // dropped: Default trait
struct Model {
    model: Mlp,
    target: Mlp,
    optimizer: Adam<Mlp, f32, Cpu>,
    steps_since_last_merge: i32,
    survived_steps: i32,
    episode: i32,
    epsilon: f32,
    experience: Vec<Transition>,
}

impl Model {
    pub fn default() -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        let mut mlp = Mlp::new(); // let mut mlp = Mlp::default();
        let mut target = Mlp::default();
        mlp.reset_params(&mut rng);
        target.reset_params(&mut rng);
        Self {
            model: mlp,
            target, // not sure what target is. does this refer to the two-stage model division?
            optimizer: Adam::default(),
            steps_since_last_merge: 0, // what is merge? 
            survived_steps: 0,
            episode: 0,
            epsilon: 1.,
            experience: Vec::new(),
        }
    }

    pub fn push_experience(&mut self, transition: Transition) {
        self.experience.push(transition); // what is transition!? why is it considered the experience?
        if self.experience.len() > 10000 { // manage size (why?)
            self.experience = self.experience[5000..].to_vec();
        }
    }

    pub fn train(&mut self) {
        // Select the experience batch
        let mut rng = rand::thread_rng();
        let distribution = rand::distributions::Uniform::from(0..self.experience.len());
        let experience: Vec<Transition> = (0..BATCH_SIZE)
            .map(|_index| self.experience[distribution.sample(&mut rng)])
            .collect(); // need to understand this statement. (may be useful to read Python code first)

        // Get the models expected rewards
        let observations: Vec<_> = experience.iter().map(|x| x.0.to_owned()).collect();
        let observations: [[f32; 4]; BATCH_SIZE] = observations.try_into().unwrap();
        let observations: Tensor2D<BATCH_SIZE, 4> = TensorCreator::new(observations);
        
        let predictions = self.model.forward(observations.trace()); // what is trace ?
        let actions_indices: Vec<_> = experience.iter().map(|x| x.1 as usize).collect();
        let actions_indices: [usize; BATCH_SIZE] = actions_indices.try_into().unwrap();
        let predictions: Tensor1D<BATCH_SIZE, dfdx::prelude::OwnedTape<f32, Cpu>> = 
            predictions.select(&actions_indices);

        // Get the targets expected rewards for the next_observation
        // This could be optimized but I can't think of a easy way to do it without making this 
        // code much more gross, and since we are already far faster than we need to be, this is
        // fine BUT when not rendering the window, this is the bottleneck in the program
        let mut target_predictions: [f32; BATCH_SIZE] = [0.; BATCH_SIZE];
        for (i) in experience.iter().enumerate() {
            let target_prediction = match x.3 {
                Some(next_observation) => {
                    let next_observation: Tensor1D<4> = TensorCreator::new(next_observation);
                    let target_prediction = self.target.forward(next_observation);
                    let target_prediction =
                        target_prediction.data()[0].max(target_prediction.data()[1]);
                    target_prediction * NEXT_STATE_DISCOUNT + experience[i].2 as f32
                }
                None => experience[i].2 as f32,
            };
        }

    }
}