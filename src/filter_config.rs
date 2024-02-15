use crate::channel::Command;

#[derive(Debug)]
pub enum FilterState {
    Normal,
    Bypass,
}

impl FilterState {
    fn to_command(&self) -> String {
        match self {
            FilterState::Normal => "0".to_string(),
            FilterState::Bypass => "1".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct FilterConfig {
    preemphasis: FilterState,
    high_pass: FilterState,
    low_pass: FilterState,
}

impl FilterConfig {
    pub fn default() -> Self {
        FilterConfig {
            preemphasis: FilterState::Normal,
            high_pass: FilterState::Normal,
            low_pass: FilterState::Normal,
        }
    }
    pub fn preemphasis(mut self, state: FilterState) -> Self {
        self.preemphasis = state;
        self
    }
    pub fn high_pass(mut self, state: FilterState) -> Self {
        self.high_pass = state;
        self
    }
    pub fn low_pass(mut self, state: FilterState) -> Self {
        self.low_pass = state;
        self
    }
}

impl FilterConfig {
    pub fn generate_command(&self) -> Result<Command, String> {
        Ok(Command {
            command: format!(
                "AT+SETFILTER={},{},{}",
                self.preemphasis.to_command(),
                self.high_pass.to_command(),
                self.low_pass.to_command()
            ),
            expected_response: "+DMOSETFILTER: 0".to_string(),
        })
    }
}
