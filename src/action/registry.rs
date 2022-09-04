use std::any::{Any, TypeId};
use std::collections::HashMap;

use super::{Action, EventAction};

#[linkme::distributed_slice]
pub static ACTION_FACTORIES: [fn(&mut ActionRegistry)] = [..];

pub struct ActionRegistry {
    // TODO: directly use u64 of TypeId as hash?
    actions: HashMap<TypeId, HashMap<String, Box<dyn Any + Send + Sync>>>,
}

impl Default for ActionRegistry {
    fn default() -> Self {
        let mut registry = Self {
            actions: Default::default(),
        };
        for f in ACTION_FACTORIES {
            (f)(&mut registry);
        }
        registry
    }
}

impl ActionRegistry {
    pub fn register<S: 'static>(&mut self, module: &str, name: &str, action: Action<S>) {
        let actions = self.actions.entry(TypeId::of::<S>()).or_default();
        actions.insert(format!("{}::{}", module, name), Box::new(action));
    }

    pub(crate) fn get<S: 'static>(&self, id: &str) -> Option<&Action<S>> {
        self.actions
            .get(&TypeId::of::<S>())
            .and_then(|actions| actions.get(id))
            .and_then(|boxed| (&**boxed as &(dyn Any + 'static)).downcast_ref())
    }

    // TODO: any way to merge register and register_event into one method?
    pub fn register_event<S: 'static, E: 'static>(
        &mut self,
        module: &str,
        name: &str,
        action: EventAction<S, E>,
    ) {
        let actions = self.actions.entry(TypeId::of::<S>()).or_default();
        actions.insert(format!("{}::{}", module, name), Box::new(action));
    }

    pub fn get_event<S: 'static, E: 'static>(&self, id: &str) -> Option<&EventAction<S, E>> {
        self.actions
            .get(&TypeId::of::<S>())
            .and_then(|actions| actions.get(id))
            .and_then(|boxed| (&**boxed as &(dyn Any + 'static)).downcast_ref())
    }
}

#[test]
fn test_action_registry() {
    fn increment(count: u32) -> u32 {
        count + 1
    }

    let action1 = Action::new("test", "increment", increment);
    let mut registry = ActionRegistry::default();

    assert_eq!(registry.get::<u32>("test::increment"), None);

    registry.register("test", "increment", action1);
    assert_eq!(registry.get::<u32>("test::increment"), Some(&action1));
    assert_eq!(registry.get::<u32>("test::foobar"), None);
    assert_eq!(registry.get::<i32>("test::increment"), None);
}
