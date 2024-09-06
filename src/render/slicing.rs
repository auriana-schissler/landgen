#[derive(Clone)]
pub struct Slicing {
    slice_height: usize,
    final_slice_height: usize,
    pub height: usize,
    pub width: usize,
    pub slice_count: u8,
    final_slice_id: u8,
}

impl Slicing {
    pub fn new(height: usize, width: usize, slice_count: u8) -> Self {
        let slice_height = (height as f64 / slice_count as f64).ceil() as usize;
        Self {
            slice_count,
            slice_height,
            final_slice_id: slice_count - 1,
            final_slice_height: height - slice_height * (slice_count as usize - 1),
            height,
            width,
        }
    }

    #[inline(always)]
    pub fn translate_index(&self, h0: usize) -> (usize, usize) {
        (h0 / self.slice_height, h0 % self.slice_height)
    }

    #[inline(always)]
    pub fn get_slice_height(&self, slice_id: u8) -> usize {
        if slice_id == self.final_slice_id {
            self.final_slice_height
        } else {
            self.slice_height
        }
    }

    #[inline(always)]
    pub fn get_absolute_height(&self, slice_id: u8, height: usize) -> usize {
        slice_id as usize * self.slice_height + height
    }
}

#[test]
fn test_slicing() {
    let slicing = Slicing::new(101, 5, 5);
    assert_eq!(slicing.width, 5);
    assert_eq!(slicing.slice_count, 5);
    assert_eq!(slicing.get_slice_height(0), 21);
    assert_eq!(slicing.get_slice_height(4), 17);
}