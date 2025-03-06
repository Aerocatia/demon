

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Rectangle {
    pub top: i16,
    pub left: i16,
    pub bottom: i16,
    pub right: i16
}
impl Rectangle {
    pub const fn from_width_and_height(width: i16, height: i16) -> Self {
        Rectangle {
            top: 0,
            left: 0,
            bottom: height,
            right: width
        }
    }
    pub const fn width(self) -> i16 {
        self.right.wrapping_sub(self.left)
    }
    pub const fn height(self) -> i16 {
        self.bottom.wrapping_sub(self.top)
    }
    pub const fn centered_inside(self, what: Rectangle) -> Self {
        let left_offset = what.width() / 2 - self.width() / 2;
        let top_offset = what.height() / 2 - self.height() / 2;
        Rectangle {
            top: self.top + top_offset,
            left: self.left + left_offset,
            bottom: self.bottom + top_offset,
            right: self.right + left_offset,
        }
    }
    pub const fn get_aspect_ratio(self) -> f32 {
        let width = self.width();
        let height = self.height();
        assert!(width > 0 && height > 0);
        (width as f32) / (height as f32)
    }
}
