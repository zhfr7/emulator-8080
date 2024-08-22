pub enum Player {
    Player1,
    Player2
}

#[derive(Default)]
struct PlayerInputs {
    start: bool,
    shoot: bool,
    left: bool,
    right: bool,
}

#[derive(Default)]
pub struct Inputs {
    credit: bool,
    p1_inputs: PlayerInputs,
    p2_inputs: PlayerInputs,
    shift_register_data: u8
}

impl PlayerInputs {
    pub fn reset(&mut self) {
        self.start = false;
        self.shoot = false;
        self.left = false;
        self.right = false;
    }
}

impl Inputs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.credit = false;
        self.p1_inputs.reset();
        self.p2_inputs.reset();
        self.shift_register_data = 0;
    }

    pub fn set_credit(&mut self) {
        self.credit = true;
    }

    fn get_player_input(&mut self, player: Player) -> &mut PlayerInputs {
        match player {
            Player::Player1 => &mut self.p1_inputs,
            Player::Player2 => &mut self.p2_inputs,
        }
    }

    pub fn set_player_start(&mut self, player: Player) {
        self.get_player_input(player).start = true;
    }

    pub fn set_player_shoot(&mut self, player: Player) {
        self.get_player_input(player).shoot = true;
    }

    pub fn set_player_left(&mut self, player: Player) {
        self.get_player_input(player).left = true;
    }

    pub fn set_player_right(&mut self, player: Player) {
        self.get_player_input(player).right = true;
    }

    pub fn set_shift_register_data(&mut self, data: u8) {
        self.shift_register_data = data;
    }

    pub fn get_port_1(&self) -> u8 {
        let bit_0 = if self.credit { 1 } else { 0 };
        let bit_1 = if self.p2_inputs.start { 1 << 1 } else { 0 };
        let bit_2 = if self.p1_inputs.start { 1 << 2 } else { 0 };
        let bit_4 = if self.p1_inputs.shoot { 1 << 4 } else { 0 };
        let bit_5 = if self.p1_inputs.left { 1 << 5 } else { 0 };
        let bit_6 = if self.p1_inputs.right { 1 << 6 } else { 0 };

        bit_0 | bit_1 | bit_2 | 1 << 3 | bit_4 | bit_5 | bit_6
    }

    pub fn get_port_2(&self) -> u8 {
        let bit_4 = if self.p2_inputs.shoot { 1 << 4 } else { 0 };
        let bit_5 = if self.p2_inputs.left { 1 << 5 } else { 0 };
        let bit_6 = if self.p2_inputs.right { 1 << 6 } else { 0 };

        bit_4 | bit_5 | bit_6
    }

    pub fn get_port_3(&self) -> u8 {
        self.shift_register_data
    }
}
