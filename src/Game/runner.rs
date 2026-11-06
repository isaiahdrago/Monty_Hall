mod puzzle;

pub struct Runner {
    puzzle: puzzle::Puzzle,
    pub iterations: usize,
    switch_times: usize,
    current_iteration: usize,
    pub wins: usize,
    did_switch: bool,
    pub switch_wins: usize,
    pub switch_games: usize,
    pub stay_wins: usize,
    pub stay_games: usize,
}

impl Runner {
    pub fn new(iterations: usize, switch_times: usize) -> Self {
        Self {
            puzzle: puzzle::Puzzle::new(),
            iterations,
            switch_times,
            current_iteration: 0,
            wins: 0,
            did_switch: false,
            switch_wins: 0,
            switch_games: 0,
            stay_wins: 0,
            stay_games: 0,
        }
    }

    pub fn run(&mut self) -> bool {
        for _ in 0..self.iterations {
            self.puzzle.reset();
            let initial_choice = rand::random_range(0..3);
            self.did_switch = false;
            self.puzzle.set_initial_choice(initial_choice);

            self.puzzle.reveal();
            if self.switch_times != 0 && self.current_iteration % self.switch_times == 0 {
                self.puzzle.switch_choice();
                self.switch_games += 1;
                self.did_switch = true;
            } else {
                self.stay_games += 1;
            }

            if self.puzzle.end_game() {
                self.wins += 1;
                if self.did_switch {
                    self.switch_wins += 1;
                } else {
                    self.stay_wins += 1;
                }
            }

            self.current_iteration += 1;
        }
        true
    }
}
