use tag_structs::{ItemCollection, ItemCollectionPermutation};
use crate::random::lcg::LCGRandomRange;
use crate::tag::{get_tag_data, get_tag_info, ReflexiveImpl, TagID};

pub unsafe fn choose_item_collection_item(item_collection_tag_id: TagID) -> TagID {
    let item_collection = get_tag_data::<ItemCollection>(item_collection_tag_id)
        .expect("choose_item_collection_item failed to get the item collection tag");

    let permutations = item_collection.permutations.as_slice();

    // Do a once-over, checking that the weights are valid
    for (index, permutation) in permutations.iter().enumerate() {
        let weight = permutation.weight;
        if weight < 1.0 || !weight.is_finite() || weight > 65535.0 {
            let name = get_tag_info(item_collection_tag_id).expect("*screams*").get_tag_path();
            panic!("Item #{index} of item_collection tag {name} has an invalid weight {weight}");
        }
        let weight_integer = weight.to_int_unchecked::<u16>();
        if weight_integer as f32 != weight {
            let name = get_tag_info(item_collection_tag_id).expect("*screams*").get_tag_path();
            panic!("Item #{index} of item_collection tag {name} has a non-integer weight {weight}");
        }
    }

    fn iterate_permutations_with_integer_weights(permutations: &'static [ItemCollectionPermutation]) -> impl Iterator<Item = (u16, TagID)> {
        permutations
            .iter()
            .enumerate()
            .map(move |(index, permutation)| {
                let weight = permutation.weight;
                // SAFETY: we just checked that it's in range of a u16 above
                let weight_integer = unsafe { weight.to_int_unchecked::<u16>() };
                (weight as u16, permutation.item.tag_id.into())
            })
    }

    let Some(total_weight) = iterate_permutations_with_integer_weights(permutations)
        .map(|p| Some(p.0))
        .reduce(|a, b| a?.checked_add(b?)) else {
        return TagID::NULL
    };

    let Some(total_weight) = total_weight else {
        let name = get_tag_info(item_collection_tag_id).expect("*screams*").get_tag_path();
        panic!("item_collection tag {name} has above 65535 weight");
    };

    let random_weight = u16::lcg_global_random_range(0, total_weight);
    let mut weight_so_far = 0u16;
    for (weight, reference) in iterate_permutations_with_integer_weights(permutations) {
        weight_so_far += weight;
        if weight_so_far > random_weight {
            return reference
        }
    }

    let name = get_tag_info(item_collection_tag_id).expect("*screams*").get_tag_path();
    unreachable!("item_collection tag {name} somehow could not calculate a random item; this is a bug!");
}
