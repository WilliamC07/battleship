pub mod game {
    pub struct Game {
        state: GameState
    }

    impl Game {
        pub fn new() -> Self {
            Game{
                state: GameState::Start(StartMenu::new())
            }
        }

        pub fn get_state(&mut self) -> &mut GameState {
            &mut self.state
        }
    }

    /// States:
    /// Start: When the user first launches the program
    /// Playing: When the user is in a game playing
    ///
    /// State Machine:
    /// Start -> Playing
    ///     ^    |
    ///     +----+
    pub enum GameState {
        Start(StartMenu),
        Playing(Playing)
    }

    pub struct StartMenu {
        state: StartMenuState,
        name: String,
        server_ip: String
    }

    #[derive(Debug, PartialEq)]
    pub enum StartMenuState {
        EnterName,
        EnterServer,
        StartServer
    }

    impl StartMenu {
        fn new() -> Self {
            StartMenu {
                state: StartMenuState::EnterName,
                name: String::new(),
                server_ip: String::new()
            }
        }

        pub fn get_state(&self) -> &StartMenuState {
            &self.state
        }

        pub fn transition_next_state(&mut self) {
            match &self.state {
                StartMenuState::EnterName => self.state = StartMenuState::StartServer,
                StartMenuState::StartServer => self.state = StartMenuState::EnterServer,
                StartMenuState::EnterServer => self.state = StartMenuState::EnterName,
            };
        }

        pub fn record_key_stroke(&mut self, key: char) {
            match &self.state {
                StartMenuState::EnterName => self.name.push(key),
                StartMenuState::EnterServer => self.server_ip.push(key),
                StartMenuState::StartServer => {}
            }
        }

        pub fn record_backspace(&mut self) {
            match &self.state {
                StartMenuState::EnterName => self.name.pop(),
                StartMenuState::EnterServer => self.server_ip.pop(),
                StartMenuState::StartServer => None
            };
        }

        pub fn get_name(&self) -> &str {
            self.name.as_str()
        }

        pub fn get_server_ip(&self) -> &str {
            self.server_ip.as_str()
        }
    }

    pub struct Playing {

    }
}
