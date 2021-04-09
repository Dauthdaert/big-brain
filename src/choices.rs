use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    actions::{Action, ActionState},
    scorers::{Scorer, Score},
    thinker::{ActionEnt, ScorerEnt},
};

// Contains different types of Considerations and Actions
#[derive(Debug)]
pub struct Choice {
    pub scorer: ScorerEnt,
    pub action_state: ActionEnt,
}
impl Choice {
    pub fn calculate(&self, scores: &Query<&Score>) -> f32 {
        scores.get(self.scorer.0).expect("Where did the score go?").0
    }
}

#[derive(Debug, Deserialize)]
pub struct ChoiceBuilder {
    pub when: Box<dyn Scorer>,
    pub then: Box<dyn Action>,
}
impl ChoiceBuilder {
    pub fn build(self, actor: Entity, cmd: &mut Commands) -> Choice {
        let action = self.then;
        Choice {
            scorer: self.when.build(actor, cmd),
            action_state: ActionState::build(action, actor, cmd),
        }
    }
}
