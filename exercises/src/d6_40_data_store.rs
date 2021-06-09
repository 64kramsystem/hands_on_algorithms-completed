use crate::d6_39_id_generator::GenData;

// We always need generation data; e.g. a drop of an old generation of and object, must not drop it!
//
pub trait EcsStore<T> {
    fn add(&mut self, g: GenData, t: T);
    fn drop(&mut self, g: GenData);
    fn get(&self, g: GenData) -> Option<&T>;
    fn get_mut(&mut self, g: GenData) -> Option<&mut T>;
    fn for_each<F: FnMut(GenData, &T)>(&self, f: F);
    fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, f: F);
}

pub struct VecStore<T> {
    items: Vec<Option<(u64, T)>>,
}

impl<T> VecStore<T> {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
}

impl<T> EcsStore<T> for VecStore<T> {
    fn add(&mut self, g: GenData, t: T) {
        if g.pos >= self.items.len() {
            self.items.resize_with(g.pos + 1, || None);
        }
        self.items[g.pos] = Some((g.gen, t));
    }

    fn drop(&mut self, g: GenData) {
        if let Some(i) = self.items.get_mut(g.pos) {
            if let Some((ig, _)) = i {
                if *ig == g.gen {
                    *i = None;
                }
            }
        }
    }

    fn get(&self, g: GenData) -> Option<&T> {
        if let Some(Some((ig, d))) = self.items.get(g.pos) {
            if *ig == g.gen {
                return Some(d);
            }
        }

        None
    }

    fn get_mut(&mut self, g: GenData) -> Option<&mut T> {
        if let Some(Some((ig, d))) = self.items.get_mut(g.pos) {
            if *ig == g.gen {
                return Some(d);
            }
        }

        None
    }

    fn for_each<F: FnMut(GenData, &T)>(&self, mut f: F) {
        for (pos, item) in self.items.iter().enumerate() {
            if let Some((gen, d)) = item {
                let g = GenData { pos, gen: *gen };
                f(g, d)
            }
        }
    }

    fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, mut f: F) {
        for (pos, item) in self.items.iter_mut().enumerate() {
            if let Some((gen, d)) = item {
                let g = GenData { pos, gen: *gen };
                f(g, d)
            }
        }
    }
}
