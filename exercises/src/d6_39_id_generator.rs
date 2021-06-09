#[derive(Clone, Copy, PartialEq)]
pub struct GenData {
    pub pos: usize,
    pub gen: u64,
}

pub struct EntityActive {
    active: bool,
    gen: u64,
}

pub struct GenManager {
    items: Vec<EntityActive>,
    drops: Vec<usize>,
}

impl GenManager {
    pub fn new() -> Self {
        Self {
            items: vec![],
            drops: vec![],
        }
    }

    pub fn next(&mut self) -> GenData {
        match self.drops.pop() {
            Some(pos) => {
                let ea = &mut self.items[pos];
                ea.active = true;
                ea.gen += 1;

                GenData { pos, gen: ea.gen }
            }
            None => {
                self.items.push(EntityActive {
                    active: true,
                    gen: 0,
                });

                GenData {
                    pos: self.items.len() - 1,
                    gen: 0,
                }
            }
        }
    }

    pub fn drop(&mut self, g: GenData) {
        // Perform bounds checking
        if let Some(ea) = self.items.get_mut(g.pos) {
            // Don't drop newer items than the passed one
            if ea.active && ea.gen == g.gen {
                ea.active = false;
                self.drops.push(g.pos);
            }
        }
    }
}
