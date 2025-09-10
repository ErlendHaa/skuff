use super::Entity;
use super::State;
use colored::Colorize;

pub fn print(events: &State) {
    for entity in events.iter().rev() {
        match entity {
            Entity::Login { id, timestamp } => {
                let timestamp = timestamp.format("%Y-%m-%d %H:%M");
                println!("{}", id.to_string().yellow());
                println!("Login @ {}", timestamp);
            }
            Entity::Logout { id, timestamp } => {
                let timestamp = timestamp.format("%Y-%m-%d %H:%M");
                println!("{}", id.to_string().yellow());
                println!("Logout @ {}", timestamp);
            }
            Entity::Break {
                id,
                timestamp,
                duration,
                autoinsert,
            } => {
                let timestamp = timestamp.format("%Y-%m-%d %H:%M");
                println!("{}", id.to_string().yellow());
                print!("Break @ {} for {}", timestamp, duration);
                if *autoinsert {
                    print!(" (auto)");
                }
                println!();
            }
            Entity::Activity {
                id,
                timestamp,
                duration,
                value,
                autoinsert,
            } => {
                let timestamp = timestamp.format("%Y-%m-%d %H:%M");
                println!("{}", id.to_string().yellow());
                print!("Activity: {} @ {} for {}", value, timestamp, duration);
                if *autoinsert {
                    print!(" (auto)");
                }
                println!();
            }
        }

        println!();
    }
}
