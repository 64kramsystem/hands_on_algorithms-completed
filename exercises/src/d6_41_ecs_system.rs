use termion::terminal_size;

use crate::d6_40_data_store::EcsStore;

pub struct Strength {
    pub s: i16,
    pub h: i16,
}

#[derive(PartialEq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

pub struct Dir {
    pub vx: i32,
    pub vy: i32,
}

pub fn move_sys<D: EcsStore<Dir>, P: EcsStore<Pos>>(dir_store: &D, pos_store: &mut P) {
    pos_store.for_each_mut(|gd, pos| {
        if let Some(dir) = dir_store.get(gd) {
            pos.x += dir.vx;
            pos.y += dir.vy;
        }
    });
}

pub fn dir_sys<D: EcsStore<Dir>, P: EcsStore<Pos>>(dir_store: &mut D, pos_store: &P) {
    let (width, height) = terminal_size().unwrap();
    let (width, height) = (width as i32, height as i32);

    dir_store.for_each_mut(|dir_gd, dir| {
        match fastrand::u8(..5) {
            0 => dir.vx += 1,
            1 => dir.vx -= 1,
            2 => dir.vy += 1,
            3 => dir.vy -= 1,
            4 => {}
            _ => unreachable!(),
        }

        if let Some(pos) = pos_store.get(dir_gd) {
            if pos.x < 4 {
                dir.vx = 1
            }
            if pos.y < 4 {
                dir.vy = 1
            }
            if pos.x > width - 4 {
                dir.vx = -1;
            }
            if pos.y > height - 4 {
                dir.vy = -1;
            }
        }

        dir.vx = dir.vx.clamp(-3, 3);
        dir.vy = dir.vy.clamp(-3, 3);

        // etc. etc.
    });
}

// Unoptimized.
//
pub fn collision_sys<P: EcsStore<Pos>, S: EcsStore<Strength>>(
    pos_store: &P,
    strength_store: &mut S,
) {
    let mut collisions = vec![];

    pos_store.for_each(|pos1_gd, pos1| {
        pos_store.for_each(|pos2_gd, pos2| {
            if pos1 == pos2 && pos1_gd != pos2_gd {
                collisions.push((pos1_gd, pos2_gd));
            }
        });
    });

    for (pos1_gd, pos2_gd) in collisions {
        let damage = match strength_store.get(pos1_gd) {
            Some(b) => b.s,
            None => continue,
        };
        let h_up = if let Some(bumpee) = strength_store.get_mut(pos2_gd) {
            let n = bumpee.s + 1;
            bumpee.h -= damage;
            if bumpee.h <= 0 {
                n
            } else {
                0
            }
        } else {
            0
        };
        if h_up > 0 {
            if let Some(bumper) = strength_store.get_mut(pos1_gd) {
                bumper.h += h_up;
                bumper.s += 1;
            }
        }
    }
}
