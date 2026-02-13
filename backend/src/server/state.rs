use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::onitama::game::{ActError, Action, Game};

pub struct State {
    id_to_game: Mutex<HashMap<String, Arc<Instance>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            id_to_game: Mutex::default(),
        }
    }
}

pub struct Instance {
    game: Mutex<Game>,
}

impl Instance {
    fn new() -> Self {
        Self {
            game: Mutex::default(),
        }
    }
    pub fn act(&self, action: &Action) -> Result<(), ActError> {
        self.game
            .lock()
            .expect("We should not poision any mutex")
            .act(action)
    }

    pub fn clone_game(&self) -> Game {
        self.game
            .lock()
            .expect("We should not poision any mutex")
            .clone()
    }
}

impl State {
    pub fn make_or_get(&self, id: String) -> Arc<Instance> {
        self.id_to_game
            .lock()
            .expect("We should never poision this lock")
            .entry(id)
            .or_insert_with(|| Arc::new(Instance::new()))
            .clone()
    }
}
