type Subscription<State> = fn(&State);
type Reducer<State, Action> = fn(&State, &Action) -> State;

pub struct Store<State, Action> {
    state: State,
    reducer: Reducer<State, Action>,
    subscriptions: Vec<Subscription<State>>,
}

impl<State, Action> Store<State, Action> {
    pub fn new(reducer: Reducer<State, Action>, initial_state: State) -> Self {
        Store {
            state: initial_state,
            reducer,
            subscriptions: vec![],
        }
    }

    pub fn dispatch(&mut self, action: Action) {
        let reducer = self.reducer;

        self.state = reducer(&self.state, &action);

        for sub in &self.subscriptions {
            sub(&self.state);
        }
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn subscribe(&mut self, sub: Subscription<State>) {
        self.subscriptions.push(sub);
    }

    pub fn replace_reducer(&mut self, next_reducer: Reducer<State, Action>) {
        self.reducer = next_reducer;
    }
}
