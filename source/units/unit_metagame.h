#ifndef DEMON_UNIT_METAGAME_H
#define DEMON_UNIT_METAGAME_H

#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"

enum {
    _unit_metagame_type_brute,
    _unit_metagame_type_grunt,
    _unit_metagame_type_jackal,
    _unit_metagame_type_skirmisher,
    _unit_metagame_type_marine,
    _unit_metagame_type_spartan,
    _unit_metagame_type_bugger,
    _unit_metagame_type_hunter,
    _unit_metagame_type_flood_infection,
    _unit_metagame_type_flood_carrier,
    _unit_metagame_type_flood_combat,
    _unit_metagame_type_flood_pure,
    _unit_metagame_type_sentinel,
    _unit_metagame_type_elite,
    _unit_metagame_type_engineer,
    _unit_metagame_type_mule,
    _unit_metagame_type_turret,
    _unit_metagame_type_mongoose,
    _unit_metagame_type_warthog,
    _unit_metagame_type_scorpion,
    _unit_metagame_type_hornet,
    _unit_metagame_type_pelican,
    _unit_metagame_type_revenant,
    _unit_metagame_type_seraph,
    _unit_metagame_type_shade,
    _unit_metagame_type_watchtower,
    _unit_metagame_type_ghost,
    _unit_metagame_type_chopper,
    _unit_metagame_type_mauler,
    _unit_metagame_type_wraith,
    _unit_metagame_type_banshee,
    _unit_metagame_type_phantom,
    _unit_metagame_type_scarab,
    _unit_metagame_type_guntower,
    _unit_metagame_type_tuning_fork,
    _unit_metagame_type_broadsword,
    _unit_metagame_type_mammoth,
    _unit_metagame_type_lich,
    _unit_metagame_type_mantis,
    _unit_metagame_type_wasp,
    _unit_metagame_type_phaeton,
    _unit_metagame_type_bishop,
    _unit_metagame_type_knight,
    _unit_metagame_type_pawn,
    NUMBER_OF_UNIT_METAGAME_TYPES
};

enum {
    _unit_metagame_class_infantry,
    _unit_metagame_class_leader,
    _unit_metagame_class_hero,
    _unit_metagame_class_specialist,
    _unit_metagame_class_light_vehicle,
    _unit_metagame_class_heavy_vehicle,
    _unit_metagame_class_giant_vehicle,
    _unit_metagame_class_standard_vehicle,
    NUMBER_OF_UNIT_METAGAME_CLASSES
};

struct unit_metagame_properties {
    int16_t metagame_type;
    int16_t metagame_class;
};
static_assert(sizeof(struct unit_metagame_properties) == 4);

#endif
