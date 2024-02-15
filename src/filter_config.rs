#[derive(Debug)]
enum FilterState {
    Normal,
    Bypass,
}

#[derive(Debug)]
pub struct FilterConfig {
    preemphasis: FilterState,
    high_pass: FilterState,
    low_pass: FilterState,
}

impl FilterConfig {
    fn default() -> Self {
        FilterConfig {
            preemphasis: FilterState::Normal,
            high_pass: FilterState::Normal,
            low_pass: FilterState::Normal,
        }
    }
    fn preemphasis(mut self, state: FilterState) -> Self {
        self.preemphasis = state;
        self
    }
    fn high_pass(mut self, state: FilterState) -> Self {
        self.high_pass = state;
        self
    }
    fn low_pass(mut self, state: FilterState) -> Self {
        self.low_pass = state;
        self
    }
    
}
