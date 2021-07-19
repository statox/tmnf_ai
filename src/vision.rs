use scrap::*;
use std::io::ErrorKind::WouldBlock;
// use std::thread;
// use std::time::Duration;

pub struct Vision {
    capturer: Capturer,
}

impl Vision {
    pub fn new() -> Vision {
        // let new_capturer = Capturer::new(Display::primary().expect("Couldn't find primary display.")).expect("Couldn't begin capture.");
        Vision {
            // display: Display::primary().expect("Couldn't find primary display."),
            capturer: Capturer::new(Display::primary().expect("Couldn't find primary display."))
                .expect("Couldn't begin capture."),
        }
    }

    pub fn read_speed(&mut self) -> i16 {
        let h = self.capturer.height();
        let w = self.capturer.width();
        loop {
            // Wait until there's a frame.
            let buffer = match self.capturer.frame() {
                Ok(buffer) => buffer,
                Err(error) => {
                    if error.kind() == WouldBlock {
                        // Keep spinning.
                        // thread::sleep(one_frame);
                        // continue;
                        break -2;
                    } else {
                        panic!("Error: {}", error);
                    }
                }
            };

            let stride = buffer.len() / h;

            // Get the speed
            let units = Vision::get_speed_digit(0, &buffer, stride, h, w);
            let dozens = Vision::get_speed_digit(1, &buffer, stride, h, w);
            let hundreds = Vision::get_speed_digit(2, &buffer, stride, h, w);

            if units > -1 && dozens == -1 && hundreds == -1 {
                // < 10 nominal
                // println!("{}", units);
                return units;
            } else if units > -1 && dozens > -1 && hundreds == -1 {
                // < 100 nominal
                // println!("{}{}", dozens, units);
                return 10 * dozens + units;
            } else if units > -1 && dozens > -1 && hundreds > -1 {
                // < 1000 nominal
                // println!("{}{}{}", hundreds, dozens, units);
                return 100 * hundreds + 10 * dozens + units;
            } else if units == -1 && dozens > -1 && hundreds > -1 {
                // < 1000 can't read units
                // println!("{}{}0 - error correction", hundreds, dozens);
                return 100 * hundreds + 10 * dozens;
            } else {
                // println!("Error reading speed {}, {}, {}", hundreds, dozens, units);
                return -1;
            }
        }
    }

    //   __a__
    //  |     |
    //  |f    |b
    //   __g__
    //  |     |
    //  |e    |c
    //   __d__
    fn get_speed_digit(digit_rank: u8, buffer: &Frame, stride: usize, h: usize, w: usize) -> i16 {
        let offset = match digit_rank {
            0 => 0,
            1 => 42,
            2 => 87,
            _ => panic!("Invalid digit rank {}", digit_rank),
        };
        let i_a = Vision::xy_to_i(h - 50, w - 40 - offset, stride);
        let wa = Vision::is_white([buffer[i_a], buffer[i_a + 1], buffer[i_a + 2]]);
        let i_g = Vision::xy_to_i(h - 30, w - 40 - offset, stride);
        let wg = Vision::is_white([buffer[i_g], buffer[i_g + 1], buffer[i_g + 2]]);
        let i_d = Vision::xy_to_i(h - 10, w - 40 - offset, stride);
        let wd = Vision::is_white([buffer[i_d], buffer[i_d + 1], buffer[i_d + 2]]);

        let i_c = Vision::xy_to_i(h - 20, w - 27 - offset, stride);
        let wc = Vision::is_white([buffer[i_c], buffer[i_c + 1], buffer[i_c + 2]]);
        let i_b = Vision::xy_to_i(h - 40, w - 27 - offset, stride);
        let wb = Vision::is_white([buffer[i_b], buffer[i_b + 1], buffer[i_b + 2]]);

        let i_e = Vision::xy_to_i(h - 20, w - 55 - offset, stride);
        let we = Vision::is_white([buffer[i_e], buffer[i_e + 1], buffer[i_e + 2]]);
        let i_f = Vision::xy_to_i(h - 40, w - 55 - offset, stride);
        let wf = Vision::is_white([buffer[i_f], buffer[i_f + 1], buffer[i_f + 2]]);

        let units = Vision::segments_to_int(wa, wb, wc, wd, we, wf, wg);
        return units as i16;
    }

    fn segments_to_int(wa: bool, wb: bool, wc: bool, wd: bool, we: bool, wf: bool, wg: bool) -> i8 {
        if !wa && wb && wc && !wd && !we && !wf && !wg {
            return 1;
        } else if wa && wb && !wc && wd && we && !wf && wg {
            return 2;
        } else if wa && wb && wc && wd && !we && !wf && wg {
            return 3;
        } else if !wa && wb && wc && !wd && !we && wf && wg {
            return 4;
        } else if wa && !wb && wc && wd && !we && wf && wg {
            return 5;
        } else if wa && !wb && wc && wd && we && wf && wg {
            return 6;
        } else if wa && wb && wc && !wd && !we && !wf && !wg {
            return 7;
        } else if wa && wb && wc && wd && we && wf && wg {
            return 8;
        } else if wa && wb && wc && wd && !we && wf && wg {
            return 9;
        } else if wa && wb && wc && wd && we && wf && !wg {
            return 0;
        } else {
            return -1;
        }
    }

    fn xy_to_i(y: usize, x: usize, buf_len: usize) -> usize {
        return buf_len * y + 4 * x;
    }
    fn is_white(c: [u8; 3]) -> bool {
        return c[0] > 250 && c[1] > 250 && c[2] > 250;
    }
}
