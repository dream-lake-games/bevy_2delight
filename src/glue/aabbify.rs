use bevy::prelude::*;

use crate::prelude::*;

#[derive(std::hash::Hash, Debug, PartialEq, Eq, Clone)]
pub struct Key {
    x: i32,
    y: i32,
}
impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.y.cmp(&other.y) {
            std::cmp::Ordering::Equal => self.x.cmp(&other.x),
            other => other,
        }
    }
}
impl Key {
    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn right_amt(&self, amt: i32) -> Self {
        Self {
            x: self.x + amt,
            y: self.y,
        }
    }
    fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn up_amt(&self, amt: i32) -> Self {
        Self {
            x: self.x,
            y: self.y + amt,
        }
    }
    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub(crate) fn from_pos(pos: Pos, grid_size: u32) -> Self {
        let iv = IVec2::new(pos.x.round().to_num(), pos.y.round().to_num());
        debug_assert!(iv.x % grid_size as i32 == 0);
        debug_assert!(iv.y % grid_size as i32 == 0);
        Key {
            x: iv.x / grid_size as i32,
            y: iv.y / grid_size as i32,
        }
    }
}

pub(crate) struct Pixel<T> {
    key: Key,
    payload: T,
}
impl<T> PartialEq for Pixel<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<T> PartialOrd for Pixel<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}
impl<T> Eq for Pixel<T> {}
impl<T> Ord for Pixel<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}
impl<T> Pixel<T> {
    pub(crate) fn from_pos(pos: Pos, grid_size: u32, payload: T) -> Self {
        Self {
            key: Key::from_pos(pos, grid_size),
            payload,
        }
    }
}

pub(crate) fn aabbify_make_hollow<
    T: Clone + PartialEq + Eq + std::hash::Hash + std::fmt::Debug,
    I: IntoIterator<Item = Pixel<T>>,
>(
    input: I,
) -> HashSet<T> {
    let input_presence =
        HashMap::<Key, T>::from_iter(input.into_iter().map(|e| (e.key, e.payload)));
    let mut output = HashSet::new();
    for (key, payload) in &input_presence {
        if !input_presence.contains_key(&key.left())
            || !input_presence.contains_key(&key.right())
            || !input_presence.contains_key(&key.up())
            || !input_presence.contains_key(&key.down())
        {
            output.insert(payload.clone());
        }
    }
    output
}

/// Is it possible to read this shit? _No_.
/// But it only took me like an hour to write. If it has crazy bugs, probably best to just
/// throw out and start over.
pub(crate) fn aabify_consolidate<
    T: Clone + PartialEq + Eq + std::hash::Hash + std::fmt::Debug,
    I: IntoIterator<Item = Pixel<T>>,
>(
    input: I,
) -> Vec<Vec<T>> {
    let mut input_vec = input.into_iter().collect::<Vec<_>>();
    input_vec.sort();
    let input_map =
        HashMap::<Key, T>::from_iter(input_vec.iter().map(|e| (e.key.clone(), e.payload.clone())));

    let mut groups: Vec<Vec<_>> = vec![];
    let mut claimed = HashSet::<Key>::default();

    for pixel in input_vec {
        if claimed.contains(&pixel.key) {
            continue;
        }
        let mut group = vec![pixel.payload.clone()];

        // Grow as far to the right as we can
        let mut x_reach = 1;
        while input_map.contains_key(&pixel.key.right_amt(x_reach)) {
            let reaching_key = pixel.key.right_amt(x_reach);
            claimed.insert(reaching_key.clone());
            group.push(input_map.get(&reaching_key).unwrap().clone());
            x_reach += 1;
        }
        // Grow as far up as we can
        let can_grow_up = |amt: i32| -> bool {
            for x_amt in 0..x_reach {
                let check_key = pixel.key.right_amt(x_amt).up_amt(amt);
                if !input_map.contains_key(&check_key) {
                    return false;
                }
            }
            true
        };
        let mut y_reach = 1;
        while can_grow_up(y_reach) {
            for x_amt in 0..x_reach {
                let check_key = pixel.key.right_amt(x_amt).up_amt(y_reach);
                claimed.insert(check_key.clone());
                group.push(input_map.get(&check_key).unwrap().clone());
            }
            y_reach += 1;
        }

        groups.push(group);
    }

    groups
}
