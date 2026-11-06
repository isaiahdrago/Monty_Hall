pub struct Puzzle {
    pub behind: usize,
    pub choice: Option<usize>,
    pub is_open: [bool; 3],
    pub not_revealed: Vec<usize>,
}

impl Puzzle {
    pub fn new() -> Self {
        Self {
            behind: rand::random_range(0..3),
            choice: None,
            is_open: [false; 3],
            not_revealed: vec![0, 1, 2],
        }
    }

    pub fn set_initial_choice(&mut self, choice: usize) {
        if choice < 3 {
            self.choice = Some(choice);
        }
    }

    pub fn end_game(&mut self) -> bool{
        self.choice == Some(self.behind)
    }

    pub fn switch_choice(&mut self) {
        if let Some(choice) = self.choice {
            let remaining_doors: Vec<usize> = self.not_revealed.iter()
                .filter(|&&door| door != choice)
                .cloned()
                .collect();
            if let Some(&new_choice) = remaining_doors.first() {
                self.choice = Some(new_choice);
            }
        }
    }

    pub fn open(&mut self, index: usize) {
        if index < 3 {
            self.is_open[index] = true;
            self.not_revealed.remove(index);
        }
    }

    pub fn reveal(&mut self) -> Option<usize> {
        let choice = self.choice?;

        let candidates: Vec<usize> = (0..3)
            .filter(|&door| door != choice && door != self.behind && !self.is_open[door])
            .collect();

        let revealed = candidates[rand::random_range(0..candidates.len())];
        self.open(revealed);
        Some(revealed)
    }

    pub fn reset(&mut self) {
        self.behind = rand::random_range(0..3);
        self.choice = None;
        self.is_open = [false; 3];
        self.not_revealed = vec![0, 1, 2];
    }
}
