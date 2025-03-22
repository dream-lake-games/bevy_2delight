use bevy::prelude::*;
use fixed::traits::ToFixed;

use crate::{
    fx,
    glue::{fvec::FVec2, Fx},
};

pub type HBoxMarker = u32;

/// HBOX?????
#[derive(Clone, Debug)]
pub struct HBox {
    offset: FVec2,
    size: UVec2,
    half_size: FVec2,
    marker: HBoxMarker,
}
impl HBox {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            offset: default(),
            size: UVec2::new(w, h),
            half_size: FVec2::new(fx!(w as i32) / fx!(2), fx!(h as i32) / fx!(2)),
            marker: default(),
        }
    }
    pub fn with_offset<X: ToFixed, Y: ToFixed>(mut self, x: X, y: Y) -> Self {
        self.offset.x = fx!(x);
        self.offset.y = fx!(y);
        self
    }
    pub fn with_size(mut self, w: u32, h: u32) -> Self {
        self.size.x = w;
        self.size.y = h;
        self
    }
    pub fn with_marker(mut self, marker: HBoxMarker) -> Self {
        self.marker = marker;
        self
    }

    pub fn translate(&mut self, fvec: FVec2) {
        self.offset += fvec;
    }
    pub fn translated(&self, fvec: FVec2) -> Self {
        Self {
            offset: self.offset + fvec,
            size: self.size,
            half_size: self.half_size,
            marker: self.marker,
        }
    }
    pub fn min_x(&self) -> Fx {
        self.offset.x - self.half_size.x
    }
    pub fn max_x(&self) -> Fx {
        self.offset.x + self.half_size.x
    }
    pub fn min_y(&self) -> Fx {
        self.offset.y - self.half_size.y
    }
    pub fn max_y(&self) -> Fx {
        self.offset.y + self.half_size.y
    }
    pub fn get_offset(&self) -> FVec2 {
        self.offset
    }
    pub fn get_size(&self) -> UVec2 {
        self.size
    }
    pub fn get_marker(&self) -> HBoxMarker {
        self.marker
    }
    pub fn bottom_left(&self) -> FVec2 {
        self.offset - self.half_size
    }
    pub fn top_left(&self) -> FVec2 {
        self.offset + FVec2::new(-self.half_size.x, self.half_size.y)
    }
    pub fn bottom_right(&self) -> FVec2 {
        self.offset + FVec2::new(self.half_size.x, -self.half_size.y)
    }
    pub fn top_right(&self) -> FVec2 {
        self.offset + self.half_size
    }
}

// I don't care that this is super verbose, and maybe inefficient. I want it to be correct.
// Can performance engineer later if needed.
impl HBox {
    /// Manhattan distance to another hitbox
    pub fn manhattan_distance(&self, rhs: &Self) -> Fx {
        let my_x_min = self.min_x();
        let my_x_max = self.max_x();
        let my_y_min = self.min_y();
        let my_y_max = self.max_y();

        let o_x_min = rhs.min_x();
        let o_x_max = rhs.max_x();
        let o_y_min = rhs.min_y();
        let o_y_max = rhs.max_y();

        let x_dist = (my_x_min - o_x_max).abs().min((o_x_min - my_x_max).abs());
        let y_dist = (my_y_min - o_y_max).abs().min((o_y_min - my_y_max).abs());

        x_dist + y_dist
    }

    /// Manhattan distance to a point
    pub fn manhattan_distance_to_point(&self, point: FVec2) -> Fx {
        let my_x_min = self.min_x();
        let my_x_max = self.max_x();
        let my_y_min = self.min_y();
        let my_y_max = self.max_y();

        let x_dist = if point.x >= my_x_min && point.x <= my_x_max {
            Fx::ZERO
        } else {
            (my_x_min - point.x).abs().min((point.x - my_x_max).abs())
        };

        let y_dist = if point.y >= my_y_min && point.y <= my_y_max {
            Fx::ZERO
        } else {
            (my_y_min - point.y).abs().min((point.y - my_y_max).abs())
        };

        x_dist + y_dist
    }

    /// Area overlapping with another hitbox
    /// NOTE: Assumes they are overlapping
    pub fn area_overlapping_assuming_overlap(&self, rhs: &Self) -> Fx {
        let my_x_min = self.min_x();
        let my_x_max = self.max_x();
        let my_y_min = self.min_y();
        let my_y_max = self.max_y();

        let ox_min = rhs.min_x();
        let ox_max = rhs.max_x();
        let oy_min = rhs.min_y();
        let oy_max = rhs.max_y();

        let x_overlap = (my_x_min - ox_max).abs().min((ox_min - my_x_max).abs());
        let y_overlap = (my_y_min - oy_max).abs().min((oy_min - my_y_max).abs());

        x_overlap * y_overlap
    }

    /// Returns if the two hitboxes overlap
    pub fn overlaps_with(&self, rhs: &Self) -> bool {
        let my_x_min = self.min_x();
        let my_x_max = self.max_x();
        let my_y_min = self.min_y();
        let my_y_max = self.max_y();

        let ox_min = rhs.min_x();
        let ox_max = rhs.max_x();
        let oy_min = rhs.min_y();
        let oy_max = rhs.max_y();

        let dont_overlap_x = (my_x_max <= ox_min) || (ox_max <= my_x_min);
        let dont_overlap_y = (my_y_max <= oy_min) || (oy_max <= my_y_min);

        !dont_overlap_x && !dont_overlap_y
    }

    /// If the two hitboxes overlap, return the vec that you need to move self to get it out of rhs
    pub fn get_push_out(&self, rhs: &Self) -> Option<FVec2> {
        // Hear me out: this might not be that inefficient.
        // Almost everytime we call this it returns none. Better to use simpler logic to get quick no in usual case.
        if !self.overlaps_with(rhs) {
            return None;
        }

        let my_x_min = self.min_x();
        let my_x_max = self.max_x();
        let my_y_min = self.min_y();
        let my_y_max = self.max_y();

        let ox_min = rhs.min_x();
        let ox_max = rhs.max_x();
        let oy_min = rhs.min_y();
        let oy_max = rhs.max_y();

        let needed_left_push = (ox_min - my_x_max).min(Fx::ZERO);
        let needed_right_push = (ox_max - my_x_min).max(Fx::ZERO);
        let needed_down_push = (oy_min - my_y_max).min(Fx::ZERO);
        let needed_up_push = (oy_max - my_y_min).max(Fx::ZERO);

        let needed_hor_push = if needed_left_push.abs() < needed_right_push.abs() {
            needed_left_push
        } else {
            needed_right_push
        };
        let needed_ver_push = if needed_down_push.abs() < needed_up_push.abs() {
            needed_down_push
        } else {
            needed_up_push
        };

        let push = if needed_hor_push.abs() < needed_ver_push.abs() {
            FVec2::new(needed_hor_push, Fx::ZERO)
        } else {
            FVec2::new(Fx::ZERO, needed_ver_push)
        };

        Some(push)
    }
}
