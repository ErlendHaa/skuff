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
            Event::Create { entity, .. } | Event::Edit { entity, .. } => {
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

#[cfg(test)]
mod tests {
    use chrono::TimeZone as _;

    use crate::Entity;
    use crate::Event;
    use crate::Id;
    use crate::Stream;

    use super::State;

    #[test]
    fn edit() {
        let id = Id::new();
        let mut stream = Stream::new();
        stream
            .push(Event::Create {
                id: Id::new(),
                created_at: chrono::Utc::now(),
                entity: Entity::Login {
                    id: id.clone(),
                    timestamp: chrono::Utc.with_ymd_and_hms(2023, 1, 1, 9, 0, 0).unwrap(),
                },
            })
            .unwrap();
        stream
            .push(Event::Edit {
                id: Id::new(),
                created_at: chrono::Utc::now(),
                entity: Entity::Login {
                    id: id.clone(),
                    timestamp: chrono::Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap(),
                },
            })
            .unwrap();

        let state = super::replay(&stream);

        let expected = State(vec![Entity::Login {
            id: id.clone(),
            timestamp: chrono::Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap(),
        }]);

        assert_eq!(state.0, expected.0);
    }

    #[test]
    fn edit_twice() {
        let id = Id::new();
        let mut stream = Stream::new();
        stream
            .push(Event::Create {
                id: Id::new(),
                created_at: chrono::Utc::now(),
                entity: Entity::Login {
                    id: id.clone(),
                    timestamp: chrono::Utc.with_ymd_and_hms(2023, 1, 1, 9, 0, 0).unwrap(),
                },
            })
            .unwrap();
        stream
            .push(Event::Edit {
                id: Id::new(),
                created_at: chrono::Utc::now(),
                entity: Entity::Login {
                    id: id.clone(),
                    timestamp: chrono::Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap(),
                },
            })
            .unwrap();
        stream
            .push(Event::Edit {
                id: Id::new(),
                created_at: chrono::Utc::now(),
                entity: Entity::Login {
                    id: id.clone(),
                    timestamp: chrono::Utc.with_ymd_and_hms(2023, 1, 1, 11, 0, 0).unwrap(),
                },
            })
            .unwrap();

        let state = super::replay(&stream);

        let expected = State(vec![Entity::Login {
            id: id.clone(),
            timestamp: chrono::Utc.with_ymd_and_hms(2023, 1, 1, 11, 0, 0).unwrap(),
        }]);

        assert_eq!(state.0, expected.0);
    }

    #[test]
    fn delete() {
        let id = Id::new();
        let mut stream = Stream::new();
        stream
            .push(Event::Create {
                id: Id::new(),
                created_at: chrono::Utc::now(),
                entity: Entity::Login {
                    id: id.clone(),
                    timestamp: chrono::Utc.with_ymd_and_hms(2023, 1, 1, 9, 0, 0).unwrap(),
                },
            })
            .unwrap();
        stream
            .push(Event::Delete {
                id: Id::new(),
                created_at: chrono::Utc::now(),
                entity_id: id.clone(),
            })
            .unwrap();

        let state = super::replay(&stream);

        let expected = State(vec![]);

        assert_eq!(state.0, expected.0);
    }

    #[test]
    fn delete_create_with_edit() {
        let id = Id::new();
        let mut stream = Stream::new();
        stream
            .push(Event::Create {
                id: Id::new(),
                created_at: chrono::Utc::now(),
                entity: Entity::Login {
                    id: id.clone(),
                    timestamp: chrono::Utc.with_ymd_and_hms(2023, 1, 1, 9, 0, 0).unwrap(),
                },
            })
            .unwrap();
        stream
            .push(Event::Edit {
                id: Id::new(),
                created_at: chrono::Utc::now(),
                entity: Entity::Login {
                    id: id.clone(),
                    timestamp: chrono::Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap(),
                },
            })
            .unwrap();
        stream
            .push(Event::Delete {
                id: Id::new(),
                created_at: chrono::Utc::now(),
                entity_id: id.clone(),
            })
            .unwrap();

        let state = super::replay(&stream);
        let expected = State(vec![]);

        assert_eq!(state.0, expected.0);
    }
}
