use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use crate::onitama::game::Game;

struct State {
    id_to_game: Mutex<HashMap<String, Arc<Instance>>>,
}

struct Instance {
    game: Mutex<Game>,
}

impl Instance {
    fn new() -> Self {
        Self {
            game: Mutex::default(),
        }
    }
}

impl State {
    fn make_or_get(&self, id: String) -> Arc<Instance> {
        self.id_to_game
            .lock()
            .expect("We should never poision this lock")
            .entry(id)
            .or_insert_with(|| Arc::new(Instance::new()))
            .to_owned()
    }
}
