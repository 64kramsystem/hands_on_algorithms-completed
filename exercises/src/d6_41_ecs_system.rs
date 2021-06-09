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
