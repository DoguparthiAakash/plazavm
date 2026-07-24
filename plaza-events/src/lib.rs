//! # plaza-events
//!
//! Asynchronous event bus for PlazaVM.
//!
//! Uses tokio broadcast channels for decoupled publish/subscribe
//! communication between all subsystems. Events are the nervous system
//! of the platform — every state change emits an event.

mod bus;
mod events;

pub use bus::EventBus;
pub use events::PlazaEvent;
