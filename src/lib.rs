/*
Copyright (c) 2020 Todd Stellanova
LICENSE: BSD3 (see LICENSE file)
*/
#![cfg_attr(not(test), no_std)]

//! Allows viewing a portion of an image, stored in a slice,
//! as a smaller image, without copying data.
//!
use core::ops::{Index};

/// Used to specifiy cols x rows
#[derive(Copy, Clone, Debug, Default)]
pub struct ImageDimensions  {
    columns: usize,
    rows: usize
}

impl ImageDimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            columns: width,
            rows: height,
        }
    }
}

pub struct SliceView<'a, T> {
    pub parent_dims: ImageDimensions,
    pub child_dims: ImageDimensions,
    parent_start_col: usize,
    parent_start_row: usize,
    slice: &'a [T],
}

impl<'a, T> SliceView<'a, T> {
    pub fn new( parent_dims: ImageDimensions, parent_start_row: usize, parent_start_col: usize, slice: &'a [T], child_dims: ImageDimensions) -> Self {
        Self {
            parent_dims,
            child_dims,
            parent_start_col,
            parent_start_row,
            slice
        }
    }
}

impl<'a, T> Index<usize> for SliceView<'a, T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        let child_y = idx / self.child_dims.columns;
        let child_x = idx % self.child_dims.columns;
        let frame_x = self.parent_start_col + child_x;
        let frame_y = self.parent_start_row + child_y;
        let frame_idx = frame_y * self.parent_dims.columns + frame_x;
        &self.slice[frame_idx]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    const FRAME_64_DIM: usize = 8;
    const FRAME_64: [u8; FRAME_64_DIM * FRAME_64_DIM] = [
        10, 20, 30, 40, 50, 60, 70, 80,
        11, 21, 31, 41, 51, 61, 71, 81,
        12, 22, 32, 42, 52, 62, 72, 82,
        13, 23, 33, 43, 53, 63, 73, 83,
        14, 24, 34, 44, 54, 64, 74, 84,
        15, 25, 35, 45, 55, 65, 75, 85,
        16, 26, 36, 46, 56, 66, 76, 86,
        17, 27, 37, 47, 57, 67, 77, 87 ];

    #[test]
    fn basic_view() {
        let parent = ImageDimensions::new(FRAME_64_DIM,FRAME_64_DIM);
        const CHILD_COLS: usize = 3;
        const CHILD_ROWS: usize = 2;
        let child = ImageDimensions::new(CHILD_COLS,CHILD_ROWS);

        //precalculate the row and column of the slice start index
        let parent_start_row = 1;
        let parent_start_col = 2;

        let view = SliceView::new(parent, parent_start_row, parent_start_col, &FRAME_64, child);

        let slice_start_idx = parent_start_row*FRAME_64_DIM + parent_start_col;
        assert_eq!(view[0], FRAME_64[slice_start_idx]); // top-left of child: 31
        assert_eq!(view[CHILD_COLS*CHILD_ROWS - 1], 52); // bottom-right of child: 52
    }

    #[test]
    fn overwrap() {
        let parent = ImageDimensions::new(FRAME_64_DIM,FRAME_64_DIM);
        const CHILD_COLS: usize = 3;
        const CHILD_ROWS: usize = 3;
        let child = ImageDimensions::new(CHILD_COLS,CHILD_ROWS);

        //precalculate the row and column of the slice start index
        let parent_start_row = 0;
        let parent_start_col = 7;

        let view = SliceView::new(parent, parent_start_row, parent_start_col, &FRAME_64, child);

        let slice_start_idx = parent_start_row*FRAME_64_DIM + parent_start_col;
        assert_eq!(view[0], FRAME_64[slice_start_idx]); // top-left of child: 80
        assert_eq!(view[0], 80); // top-left of child: 80
        assert_eq!(view[CHILD_COLS*CHILD_ROWS - 1], 23); // bottom-right of child: 23
    }
}
