use tag_structs::Scenario;
use crate::tag::ReflexiveImpl;

pub mod c;

pub unsafe fn ai_index_from_string(scenario: &mut Scenario, ai_string: &str) -> Option<u32> {
    if ai_string.eq_ignore_ascii_case("none") {
        return Some(0xFFFFFFFF)
    }

    let get_encounter_by_name = |name: &str| {
        for (encounter_index, encounter_data) in scenario.encounters.iter().enumerate() {
            if encounter_data.name.to_str().eq_ignore_ascii_case(name) {
                return Some((encounter_index as u32) & 0xFFFF)
            }
        }
        None
    };

    let Some((encounter, subencounter)) = ai_string.split_once('/') else {
        let Some(encounter_index) = get_encounter_by_name(ai_string) else { return None };
        return Some(encounter_index)
    };

    let Some(encounter_index) = get_encounter_by_name(encounter) else { return None };
    let encounter = scenario.encounters.get(encounter_index as usize).expect("got encounter index earlier");

    let upper = if let Some(squad_index) = encounter.squads.iter().position(|p| p.name.to_str().eq_ignore_ascii_case(subencounter)) {
        (0x8000 | (squad_index & 0xFF)) << 16
    }
    else if let Some(platoon_index) = encounter.platoons.iter().position(|p| p.name.to_str().eq_ignore_ascii_case(subencounter)) {
        (0x4000 | (platoon_index & 0xFF)) << 16
    }
    else {
        return None
    };

    Some((upper as u32) | (encounter_index))
}
