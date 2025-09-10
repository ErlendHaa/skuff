use std::collections::HashMap;

use super::Entity;
use super::Event;
use super::Stream;

pub struct State(Vec<Entity>);

impl State {
    pub fn iter(&self) -> std::slice::Iter<'_, Entity> {
        self.0.iter()
    }
}

pub fn replay(events: &Stream) -> State {
    let mut state = HashMap::new();

    for event in events {
        match event {
            Event::Create { entity, .. } | Event::Update { entity, .. } => {
                let id = match entity {
                    Entity::Login { id, .. }
                    | Entity::Logout { id, .. }
                    | Entity::Break { id, .. }
                    | Entity::Activity { id, .. } => id,
                };
                state.insert(id.clone(), entity.clone());
            }
            Event::Delete { entity_id, .. } => {
                state.remove(&entity_id);
            }
        }
    }
    let mut state: Vec<Entity> = state.into_values().collect();

    state.sort_by_key(|entity| match entity {
        Entity::Login { timestamp, .. }
        | Entity::Logout { timestamp, .. }
        | Entity::Break { timestamp, .. }
        | Entity::Activity { timestamp, .. } => *timestamp,
    });

    State(state)
}
