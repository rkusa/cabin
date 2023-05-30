use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::io::Cursor;
use std::ops::Deref;
use std::rc::Rc;

use serde::ser::{SerializeMap, SerializeTuple};
use serde::{Serialize, Serializer};
use tokio::sync::{mpsc, oneshot};

use crate::signal::{Signal, SignalId};

tokio::task_local! {
    static SCOPE: Scope;
}

#[derive(Clone)]
pub struct Scope {
    state: Rc<RefCell<State>>,
}

struct State {
    serialized_state: Vec<u8>,
    known_signals: HashSet<SignalId>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(State {
                serialized_state: vec![b'{'],
                known_signals: Default::default(),
            })),
        }
    }

    pub async fn run<T>(self, f: impl Future<Output = T>) -> T {
        SCOPE.scope(self, f).await
    }

    pub fn add_signal<T>(signal: &Signal<T>)
    where
        T: Serialize,
    {
        SCOPE
            .try_with(|scope| {
                let mut state = scope.state.borrow_mut();
                if state.known_signals.contains(&signal.id()) {
                    return;
                }

                if state.serialized_state.len() > 1 {
                    state.serialized_state.push(b',');
                }

                // TODO: unwrap
                let mut ser = serde_json::Serializer::new(&mut state.serialized_state);
                ser.serialize_str(signal.id()).unwrap();
                state.serialized_state.push(b':');
                let mut ser = serde_json::Serializer::new(&mut state.serialized_state);
                signal.value().serialize(&mut ser).unwrap();
            })
            .ok();
    }

    pub fn into_view(self) -> String {
        let Ok(state) = Rc::try_unwrap(self.state) else {
            // TODO: error?
            return String::new();
        };

        let mut serialized_state = state.into_inner().serialized_state;
        serialized_state.push(b'}');
        String::from_utf8(serialized_state).unwrap()
    }
}
