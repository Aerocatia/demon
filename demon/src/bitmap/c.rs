use core::ptr::null_mut;
use c_mine::c_mine;
use crate::tag::{get_tag_info_typed, ReflexiveImpl, TagID};
use tag_structs::{Bitmap, BitmapData};

#[c_mine]
pub unsafe extern "C" fn bitmap_group_get_bitmap_from_sequence(tag: TagID, sequence_index: u16, frame_index: u16) -> *mut BitmapData {
    if tag.is_null() {
        return null_mut();
    }

    let frame_index = frame_index as usize;
    let sequence_index = sequence_index as usize;

    let (bitmap_info, bitmap) = get_tag_info_typed::<Bitmap>(tag).expect("bitmap_group_get_bitmap_from_sequence with bad tag ID");

    let sequences = bitmap.bitmap_group_sequence.as_slice();
    let Some(sequence) = sequences.get(sequence_index) else {
        panic!(
            "bitmap_group_get_bitmap_from_sequence with bad sequence index {sequence_index} (bitmap {} only has {} sequence(s))",
            bitmap_info.get_tag_path(),
            sequences.len()
        );
    };

    // Prevent possible division by 0.
    if sequence.bitmap_count == 0 && sequence.sprites.is_empty() {
        panic!("bitmap_group_get_bitmap_from_sequence with bad sequence index {sequence_index} (sequence {sequence_index} is empty)");
    }

    // Provided the bitmap is not corrupted, we should be good now
    let bitmap_index = if sequence.bitmap_count == 0 {
        // Sprite sequences
        let sprites = sequence.sprites.as_slice();
        let sequence = sprites[frame_index % sprites.len()];
        let Some(index) = sequence.bitmap_index.get() else {
            // TODO: Check this on load!
            panic!(
                "bitmap {} sequence {sequence_index} frame index {frame_index} has no bitmap; this tag is corrupt",
                bitmap_info.get_tag_path()
            );
        };
        index
    }
    else {
        // Bitmap sequences
        let bitmap_count = sequence.bitmap_count as usize;
        let Some(first_index) = sequence.first_bitmap_index.get() else {
            // TODO: Check this on load!
            panic!(
                "bitmap {} sequence {sequence_index} has no first bitmap; this tag is corrupt",
                bitmap_info.get_tag_path()
            );
        };
        first_index + frame_index % bitmap_count
    };

    let bitmap_data = bitmap.bitmap_data.as_mut_slice();
    let Some(bitmap_data) = bitmap_data.get_mut(bitmap_index) else {
        panic!(
            "bitmap {} sequence {sequence_index} frame index {frame_index} references an out-of-bounds bitmap index {bitmap_index} when there are only {} bitmap data entry(s)", bitmap_data.len(),
            bitmap_info.get_tag_path()
        )
    };

    bitmap_data
}
