use enigo::*;

pub struct Controller {
    enigo: Enigo,
    up_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    called: u32,
    changed: u32,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            enigo: Enigo::new(),
            up_pressed: false,
            left_pressed: false,
            right_pressed: false,
            called: 0,
            changed: 0,
        }
    }

    pub fn press_keys(&mut self, up: bool, right: bool, left: bool) {
        self.called += 1;
        let mut changed_pressed = false;
        if up && !self.up_pressed {
            changed_pressed = true;
            self.up_pressed = true;
            self.enigo.key_down(Key::Layout('a'));
        }
        if !up && self.up_pressed {
            changed_pressed = true;
            self.up_pressed = false;
            self.enigo.key_up(Key::Layout('a'));
        }

        if right && !self.right_pressed {
            changed_pressed = true;
            self.right_pressed = true;
            self.enigo.key_down(Key::RightArrow);
        }
        if !right && self.right_pressed {
            changed_pressed = true;
            self.right_pressed = false;
            self.enigo.key_up(Key::RightArrow);
        }

        if left && !self.left_pressed {
            changed_pressed = true;
            self.left_pressed = true;
            self.enigo.key_down(Key::LeftArrow);
        }
        if !left && self.left_pressed {
            changed_pressed = true;
            self.left_pressed = false;
            self.enigo.key_up(Key::LeftArrow);
        }

        if changed_pressed {
            self.changed += 1;
        }
    }

    pub fn show_stats(self) {
        println!(
            "Called {} times, actual changed {} times",
            self.called, self.changed
        )
    }
}
