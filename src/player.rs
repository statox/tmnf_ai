use crate::controller::Controller;

pub struct Player {
    controller: Controller,
}

impl Player {
    pub fn new() -> Player {
        Player {
            controller: Controller::new(),
        }
    }

    pub fn show_controller_stats(self) {
        self.controller.show_stats();
    }

    pub fn drive(&mut self, speed: i16) {
        if speed < 350 {
            self.controller.press_keys(true, false, false);
        } else {
            self.controller.press_keys(false, false, false);
        }
    }
}
