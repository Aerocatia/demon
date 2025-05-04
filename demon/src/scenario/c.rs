use c_mine::c_mine;
use tag_structs::primitives::data::Index;
use tag_structs::{Scenario, ScenarioSquadAttacking};
use crate::tag::{get_tag_info, ReflexiveImpl, TagID};
use crate::util::CStrPtr;

const VARIANT_NUMBERS: &[(&str, u16)] = &[
    ("bisenti", 2),
    ("fitzgerald", 4),
    ("jenkins", 4),
    ("aussie", 5),
    ("mendoza", 6),
    ("sarge2", 101),
    ("sarge", 100),
    ("johnson", 100),
    ("lehto", 101),
];

#[c_mine]
pub unsafe extern "C" fn ai_index_from_string(scenario: &mut Scenario, ai_string: CStrPtr, ai_index_reference: &mut u32) -> bool {
    match super::ai_index_from_string(scenario, ai_string.expect_str()) {
        Some(n) => {
            *ai_index_reference = n;
            true
        }
        None => {
            *ai_index_reference = u32::MAX;
            false
        }
    }
}

#[c_mine]
pub unsafe extern "C" fn preprocess_encounter_indices_and_squad_memes(scenario: &mut Scenario, do_the_rest_of_the_function: bool) {
    // TODO: This function should probably be split up

    for (ai_conversation_index, ai_conversation) in scenario.ai_conversations.iter_mut().enumerate() {
        for (participant_index, participant) in ai_conversation.participants.iter_mut().enumerate() {
            let mut variant_indices: [u16; 6] = [u16::MAX; 6];

            participant.encounter_index = super::ai_index_from_string(
                scenario,
                participant.encounter_name.to_str()
            ).unwrap_or(0xFFFFFFFF);

            for (line_index, line) in ai_conversation
                .lines
                .iter()
                .enumerate()
                .filter(|i| i.1.participant.get().is_some_and(|p| p == participant_index)) {

                for (variant, reference) in [
                    line.variant_1,
                    line.variant_2,
                    line.variant_3,
                    line.variant_4,
                    line.variant_5,
                    line.variant_6
                ].iter().enumerate() {
                    let tag_id = TagID::from(reference.tag_id);
                    if tag_id.is_null() {
                        continue
                    }
                    let tag = get_tag_info(tag_id).expect("failed to get a tag ID");
                    let path = tag.get_tag_path();
                    let variant_data = VARIANT_NUMBERS
                        .iter()
                        .find(|i| path.contains(i.0)).map(|i| i.1)
                        .unwrap_or(0);

                    if variant_indices[variant] != variant_data && variant_indices[variant] != 0xFFFF {
                        warn!("Variant #{variant} of participant #{ai_conversation_index} of conversation #{ai_conversation_index} matches multiple characters!");
                    }

                    variant_indices[variant] = variant_data;
                }
            }

            participant.variant_numbers = variant_indices;
        }
    }

    for encounter in scenario.encounters.iter_mut() {
        for squad in encounter.squads.iter_mut() {
            for starting_location in squad.starting_locations.iter_mut() {
                if starting_location.command_list.get().is_some_and(|s| s >= scenario.command_lists.len()) {
                    starting_location.command_list = Index::new_none();
                }
            }
        }
    }

    if do_the_rest_of_the_function {
        return
    }

    for (encounter_index, encounter) in scenario.encounters.iter_mut().enumerate() {
        encounter.precomputed_bsp_index = Index::new_none();

        for (squad_index, squad) in encounter.squads.iter_mut().enumerate() {
            if squad.actor_type.get().is_some_and(|s| s >= scenario.actor_palette.len()) {
                panic!("Squad {squad_index} of encounter {encounter_index} has an out-of-bounds actor index");
            }

            for (move_position_index, move_position) in squad.move_positions.iter_mut().enumerate() {
                if move_position.weight <= 0.0 {
                    // TODO: We should probably just ban weights less than 0 while defaulting 0 to 0.001
                    move_position.weight = 0.001;
                }

                if move_position.animation.get().is_some_and(|a| a >= scenario.ai_animation_references.len()) {
                    panic!("Move position {move_position_index} of squad {squad_index} of encounter {encounter_index} has an out-of-bounds animation index");
                }
            }

            if squad.attacking.0 == 0 {
                // set all by default
                squad.attacking.0 = 0x3FFFFFF;
            }

            let attacking = squad.attacking;
            let default_to_attacking = |squad_position: ScenarioSquadAttacking| -> ScenarioSquadAttacking {
                if squad_position.0 == 0 {
                    attacking
                }
                else {
                    squad_position
                }
            };

            squad.attacking_search = default_to_attacking(squad.attacking_search);
            squad.attacking_guard = default_to_attacking(squad.attacking_guard);
            squad.defending = default_to_attacking(squad.defending);
            squad.defending_search = default_to_attacking(squad.defending_search); // probably an oversight; this would make more sense if it was default_to_defending
            squad.defending_guard = default_to_attacking(squad.defending_guard); // probably an oversight; this would make more sense if it was default_to_defending
            squad.attacking_search.0 |= squad.attacking.0;
            squad.defending_search.0 |= squad.defending.0;
        }
    }

    for command_list in scenario.command_lists.iter_mut() {
        command_list.precomputed_bsp_index = Index::new_none();
    }
}
