use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::Error;
use crate::Id;

pub struct Stream(Vec<Event>);

impl Stream {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_buffer(buf: &[u8]) -> Result<Self, Error> {
        let stream: _Stream =
            serde_json::from_slice(buf).map_err(|err| Error::DeserializeFailed(err.to_string()))?;

        Ok(stream.into())
    }

    pub fn to_buffer(&self) -> Result<Vec<u8>, Error> {
        serde_json::to_vec(&self.0).map_err(|err| Error::SerializeFailed(err.to_string()))
    }

    pub fn push(&mut self, event: Event) -> Result<(), Error> {
        // TODO validate event
        self.0.push(event);

        Ok(())
    }
}

impl From<_Stream> for Stream {
    fn from(value: _Stream) -> Self {
        Stream(value.0)
    }
}

impl<'a> IntoIterator for &'a Stream {
    type Item = &'a Event;
    type IntoIter = std::slice::Iter<'a, Event>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Event {
    Create {
        #[serde(rename = "event_id")]
        id: Id,
        created_at: DateTime<Utc>,
        #[serde(flatten)]
        entity: Entity,
    },
    Edit {
        #[serde(rename = "event_id")]
        id: Id,
        created_at: DateTime<Utc>,
        #[serde(flatten)]
        entity: Entity,
    },
    Delete {
        #[serde(rename = "event_id")]
        id: Id,
        created_at: DateTime<Utc>,
        entity_id: Id,
    },
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Entity {
    Login {
        #[serde(rename = "entity_id")]
        id: Id,
        timestamp: DateTime<Utc>,
    },
    Logout {
        #[serde(rename = "entity_id")]
        id: Id,
        timestamp: DateTime<Utc>,
    },
    Break {
        #[serde(rename = "entity_id")]
        id: Id,
        timestamp: DateTime<Utc>,

        #[serde(with = "duration_seconds")]
        duration: Duration,
        autoinsert: bool,
    },
    Activity {
        #[serde(rename = "entity_id")]
        id: Id,
        timestamp: DateTime<Utc>,
        #[serde(with = "duration_seconds")]
        duration: Duration,
        value: String,
        autoinsert: bool,
    },
}

#[derive(Serialize, Deserialize)]
struct _Stream(Vec<Event>);

mod duration_seconds {
    use chrono::Duration;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(duration.num_seconds())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = i64::deserialize(deserializer)?;
        Ok(Duration::seconds(secs))
    }
}

/// Although at first glance it might seem redundant to have tests for serialization and
/// deserialization, these tests are crucial for ensuring the integrity and consistency of the
/// event data. They serve several important purposes:
/// 1. **Preventing Regressions**: When changes are made to the codebase, these tests help
///    catch any unintended side effects that might alter the serialization format.
/// 2. **Confidence in Refactoring**: When refactoring code, having tests for serialization and
///    deserialization provides confidence that the changes do not break existing functionality.
/// 3. **Backward Compatibility**: As the codebase evolves, changes to the data structures may
///    occur. These tests help verify that older serialized data can still be correctly
///    deserialized, ensuring backward compatibility.
/// 4. **Documentation**: Tests serve as a form of documentation, illustrating how the data
///    structures are expected to be serialized. This can be helpful for new developers
///    joining the project or for anyone needing to understand the data format.
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use chrono::TimeZone;
    use chrono::Utc;
    use serde_json::json;

    fn fixed_time() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2025, 9, 1, 12, 0, 0).unwrap()
    }

    fn to_json<T: Serialize>(value: &T) -> serde_json::Value {
        serde_json::to_value(value).unwrap()
    }

    fn assert_json_eq(actual: &serde_json::Value, expected: &serde_json::Value) {
        if actual != expected {
            panic!(
                "JSON mismatch!\n\nExpected:\n{}\n\nActual:\n{}",
                serde_json::to_string_pretty(expected).unwrap(),
                serde_json::to_string_pretty(actual).unwrap()
            );
        }
    }

    #[test]
    fn login_create() {
        let event_id = Id::new();
        let entity_id = Id::new();
        let created_at = fixed_time();
        let timestamp = fixed_time();

        let expected = json!({
            "op": "create",
            "event_id": event_id,
            "created_at": created_at,
            "entity_id": entity_id,
            "type": "login",
            "timestamp": timestamp
        });

        let event = Event::Create {
            id: event_id,
            created_at,
            entity: Entity::Login {
                timestamp,
                id: entity_id,
            },
        };

        assert_json_eq(&to_json(&event), &expected);
    }

    #[test]
    fn logout_create() {
        let event_id = Id::new();
        let entity_id = Id::new();
        let created_at = fixed_time();
        let timestamp = fixed_time();

        let expected = json!({
            "op": "create",
            "event_id": event_id,
            "created_at": created_at,
            "entity_id": entity_id,
            "type": "logout",
            "timestamp": timestamp
        });

        let event = Event::Create {
            id: event_id,
            created_at,
            entity: Entity::Logout {
                timestamp,
                id: entity_id,
            },
        };

        assert_json_eq(&to_json(&event), &expected);
    }

    #[test]
    fn break_create() {
        let event_id = Id::new();
        let entity_id = Id::new();
        let created_at = fixed_time();
        let timestamp = fixed_time();
        let duration = Duration::seconds(600);
        let autoinsert = false;

        let expected = json!({
            "op": "create",
            "event_id": event_id,
            "created_at": created_at,
            "entity_id": entity_id,
            "type": "break",
            "timestamp": timestamp,
            "duration": 600,
            "autoinsert": autoinsert
        });

        let event = Event::Create {
            id: event_id,
            created_at,
            entity: Entity::Break {
                timestamp,
                duration,
                id: entity_id,
                autoinsert,
            },
        };

        assert_json_eq(&to_json(&event), &expected);
    }

    #[test]
    fn activity_create() {
        let event_id = Id::new();
        let entity_id = Id::new();
        let created_at = fixed_time();
        let timestamp = fixed_time();
        let duration = Duration::seconds(1200);
        let autoinsert = false;

        let expected = json!({
            "op": "create",
            "event_id": event_id,
            "created_at": created_at,
            "entity_id": entity_id,
            "type": "activity",
            "timestamp": timestamp,
            "duration": 1200,
            "value": "Coding",
            "autoinsert": autoinsert
        });

        let event = Event::Create {
            id: event_id,
            created_at,
            entity: Entity::Activity {
                id: entity_id,
                timestamp,
                duration,
                value: "Coding".to_string(),
                autoinsert,
            },
        };

        assert_json_eq(&to_json(&event), &expected);
    }

    // -------- Edit variants --------

    #[test]
    fn login_edit() {
        let event_id = Id::new();
        let entity_id = Id::new();
        let created_at = fixed_time();
        let timestamp = fixed_time();

        let expected = json!({
            "op": "edit",
            "event_id": event_id,
            "created_at": created_at,
            "entity_id": entity_id,
            "type": "login",
            "timestamp": timestamp
        });

        let event = Event::Edit {
            id: event_id,
            created_at,
            entity: Entity::Login {
                timestamp,
                id: entity_id,
            },
        };

        assert_json_eq(&to_json(&event), &expected);
    }

    #[test]
    fn logout_edit() {
        let event_id = Id::new();
        let entity_id = Id::new();
        let created_at = fixed_time();
        let timestamp = fixed_time();

        let expected = json!({
            "op": "edit",
            "event_id": event_id,
            "created_at": created_at,
            "entity_id": entity_id,
            "type": "logout",
            "timestamp": timestamp
        });

        let event = Event::Edit {
            id: event_id,
            created_at,
            entity: Entity::Logout {
                timestamp,
                id: entity_id,
            },
        };

        assert_json_eq(&to_json(&event), &expected);
    }

    #[test]
    fn break_edit() {
        let event_id = Id::new();
        let entity_id = Id::new();
        let created_at = fixed_time();
        let timestamp = fixed_time();
        let duration = Duration::seconds(900);
        let autoinsert = false;

        let expected = json!({
            "op": "edit",
            "event_id": event_id,
            "created_at": created_at,
            "entity_id": entity_id,
            "type": "break",
            "timestamp": timestamp,
            "duration": 900,
            "autoinsert": autoinsert
        });

        let event = Event::Edit {
            id: event_id,
            created_at,
            entity: Entity::Break {
                timestamp,
                duration,
                id: entity_id,
                autoinsert,
            },
        };

        assert_json_eq(&to_json(&event), &expected);
    }

    #[test]
    fn activity_edit() {
        let event_id = Id::new();
        let entity_id = Id::new();
        let created_at = fixed_time();
        let timestamp = fixed_time();
        let duration = Duration::seconds(1800);
        let autoinsert = false;

        let expected = json!({
            "op": "edit",
            "event_id": event_id,
            "created_at": created_at,
            "entity_id": entity_id,
            "type": "activity",
            "timestamp": timestamp,
            "duration": 1800,
            "value": "Review",
            "autoinsert": autoinsert
        });

        let event = Event::Edit {
            id: event_id,
            created_at,
            entity: Entity::Activity {
                id: entity_id,
                timestamp,
                duration,
                value: "Review".to_string(),
                autoinsert,
            },
        };

        assert_json_eq(&to_json(&event), &expected);
    }

    // -------- Delete variants --------

    #[test]
    fn login_delete() {
        let event_id = Id::new();
        let entity_id = Id::new();
        let created_at = fixed_time();

        let expected = json!({
            "op": "delete",
            "event_id": event_id,
            "created_at": created_at,
            "entity_id": entity_id
        });

        let event = Event::Delete {
            id: event_id,
            created_at,
            entity_id,
        };

        assert_json_eq(&to_json(&event), &expected);
    }

    #[test]
    fn logout_delete() {
        let event_id = Id::new();
        let entity_id = Id::new();
        let created_at = fixed_time();

        let expected = json!({
            "op": "delete",
            "event_id": event_id,
            "created_at": created_at,
            "entity_id": entity_id
        });

        let event = Event::Delete {
            id: event_id,
            created_at,
            entity_id,
        };

        assert_json_eq(&to_json(&event), &expected);
    }

    #[test]
    fn break_delete() {
        let event_id = Id::new();
        let entity_id = Id::new();
        let created_at = fixed_time();

        let expected = json!({
            "op": "delete",
            "event_id": event_id,
            "created_at": created_at,
            "entity_id": entity_id
        });

        let event = Event::Delete {
            id: event_id,
            created_at,
            entity_id,
        };

        assert_json_eq(&to_json(&event), &expected);
    }

    #[test]
    fn activity_delete() {
        let event_id = Id::new();
        let entity_id = Id::new();
        let created_at = fixed_time();

        let expected = json!({
            "op": "delete",
            "event_id": event_id,
            "created_at": created_at,
            "entity_id": entity_id
        });

        let event = Event::Delete {
            id: event_id,
            created_at,
            entity_id,
        };

        assert_json_eq(&to_json(&event), &expected);
    }
}
