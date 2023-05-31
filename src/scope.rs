use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::{Serialize, Serializer};
use serde_json::value::RawValue;
use twox_hash::XxHash32;

use crate::signal::{Signal, SignalId};

tokio::task_local! {
    static SCOPE: Scope;
}

#[derive(Clone)]
pub struct Scope {
    state: Rc<RefCell<State>>,
}

struct State {
    prev_state: Option<HashMap<String, Box<RawValue>>>,
    next_state: Vec<u8>,
    event: Option<Event>,
}

enum Event {
    Raw { id: u32, payload: Box<RawValue> },
    Deserialized(Box<dyn Any>),
}

pub fn event<E>() -> Option<E>
where
    E: DeserializeOwned + Copy + 'static,
{
    SCOPE
        .try_with(|scope| {
            let mut state = scope.state.borrow_mut();
            let event = state.event.as_mut()?;
            match event {
                Event::Raw { id, payload } => {
                    let mut hasher = XxHash32::default();
                    TypeId::of::<E>().hash(&mut hasher);
                    let type_id = hasher.finish() as u32;

                    if *id != type_id {
                        return None;
                    }

                    // TODO: unwrap
                    let payload: E = serde_json::from_str(payload.get()).unwrap();
                    *event = Event::Deserialized(Box::new(payload));

                    Some(payload)
                }
                Event::Deserialized(payload) => payload.downcast_ref::<E>().copied(),
            }
        })
        .ok()
        .flatten()
}

impl Scope {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(State {
                prev_state: None,
                next_state: vec![b'{'],
                event: None,
            })),
        }
    }

    pub fn with_prev_state(self, prev_state: HashMap<String, Box<RawValue>>) -> Self {
        {
            let mut state = self.state.borrow_mut();
            state.prev_state = Some(prev_state);
        }
        self
    }

    pub fn with_event(self, id: u32, payload: Box<RawValue>) -> Self {
        {
            let mut state = self.state.borrow_mut();
            state.event = Some(Event::Raw { id, payload });
        }
        self
    }

    pub async fn run<T>(self, f: impl Future<Output = T>) -> T {
        SCOPE.scope(self, f).await
    }

    pub(crate) fn restore<T>(id: SignalId) -> Option<T>
    where
        T: DeserializeOwned,
    {
        SCOPE
            .try_with(|scope| {
                let mut state = scope.state.borrow_mut();
                let prev = state.prev_state.as_mut()?.remove(id)?;

                // TODO: unwrap
                let payload: T = serde_json::from_str(prev.get()).unwrap();
                Some(payload)
            })
            .ok()
            .flatten()
    }

    pub(crate) fn serialize_signal<T>(signal: &Signal<T>)
    where
        T: Serialize,
    {
        SCOPE
            .try_with(|scope| {
                let mut state = scope.state.borrow_mut();

                if state.next_state.len() > 1 {
                    state.next_state.push(b',');
                }

                // TODO: unwrap
                let mut ser = serde_json::Serializer::new(&mut state.next_state);
                ser.serialize_str(signal.id()).unwrap();
                state.next_state.push(b':');
                let mut ser = serde_json::Serializer::new(&mut state.next_state);
                signal.value().serialize(&mut ser).unwrap();
            })
            .ok();
    }

    pub fn into_view(self) -> String {
        let Ok(state) = Rc::try_unwrap(self.state) else {
            // TODO: error?
            return String::new();
        };

        let mut serialized_state = state.into_inner().next_state;
        serialized_state.push(b'}');
        String::from_utf8(serialized_state).unwrap()
    }
}
