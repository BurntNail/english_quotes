use tui::widgets::ListState;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub fn default_state() -> ListState {
    let mut s = ListState::default();
    s.select(Some(0));
    s
}

pub fn down_arrow(state: &mut ListState, amt: usize) {
    if let Some(selected) = state.selected() {
        if selected >= amt - 1 {
            state.select(Some(0));
        } else {
            state.select(Some(selected + 1));
        }
    }
}

pub fn up_arrow(state: &mut ListState, amt: usize) {
    if let Some(selected) = state.selected() {
        if selected > 0 {
            state.select(Some(selected - 1));
        } else {
            state.select(Some(amt - 1));
        }
    }
}
