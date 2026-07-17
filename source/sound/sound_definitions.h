#ifndef DEMON_SOUND_DEFINITIONS_H
#define DEMON_SOUND_DEFINITIONS_H

#include "../cseries/cseries.h"
#include "../tag_files/tag_groups.h"

/* sound */

enum {
    SOUND_DEFINITION_TAG = 0x736E6421, // 'snd!'
    SOUND_DEFINITION_VERSION = 4,
    MAXIMUM_PROMOTION_RULES_PER_SOUND = 4,
    MAXIMUM_PITCH_RANGES_PER_SOUND = 8,
    MAXIMUM_PERMUTATIONS_PER_PITCH_RANGE = 256,
    MAXIMUM_PERMUTATIONS_PER_RANDOM_PITCH_RANGE = INT32_BITS,
    MAXIMUM_SOUND_DATA_SIZE = 4 * MIB,
    MAXIMUM_SOUND_MOUTH_DATA_SIZE = 8192,
    SOUND_MOUTH_SAMPLES_PER_SECOND = TICKS_PER_SECOND,
    MAXIMUM_SOUND_SUBTITLE_DATA_SIZE = 512,
    SOUND_COMPRESSION_BLOCK_SIZE = 64
};

enum {
    _sound_definition_fit_to_compression_block_size_bit,
    _sound_definition_linked_permutations_bit,
    NUMBER_OF_SOUND_DEFINITION_FLAGS
};

enum {
    _sound_encoding_mono,
    _sound_encoding_stereo,
    NUMBER_OF_SOUND_ENCODINGS
};

enum {
    _sound_compression_none,
    _sound_compression_xbox_adpcm,
    _sound_compression_ima_adpcm,
    _sound_compression_ogg_vorbis,
    NUMBER_OF_SOUND_COMPRESSION_TYPES
};

enum {
    _sound_sample_rate_22k,
    _sound_sample_rate_44k,
    NUMBER_OF_SOUND_SAMPLE_RATES
};

struct sound_permutation {
    char name[TAG_STRING_LENGTH + 1];
    real skip_fraction;
    real gain;
    int16_t duplicate_compression;
    int16_t next_permutation_index;
    int32_t cache_block_index;
    void *cache_base_address;
    int32_t cache_tag_index;
    int32_t unused0[1];
    int32_t runtime_tag_index;
    struct tag_data samples;
    struct tag_data mouth_data;
    struct tag_data subtitle_data;
};
static_assert(sizeof(struct sound_permutation) == 124);

struct sound_pitch_range {
    char name[TAG_STRING_LENGTH + 1];
    real natural_pitch;
    real bend_lower_bound;
    real bend_upper_bound;
    int16_t actual_permutation_count;
    uint16_t plenty_of_unused_space_here;
    real runtime_oo_natural_pitch;
    unsigned long runtime_permutation_flags;
    int16_t runtime_last_permutation_index;
    int16_t runtime_discarded_permutation_index;
    struct tag_block permutations;
};
static_assert(sizeof(struct sound_pitch_range) == 72);

struct sound_scale_modifiers {
    real skip_fraction;
    real gain;
    real pitch;
    int32_t unused0[3];
};

struct sound_definition {
    int32_t flags;
    int16_t class_index;
    int16_t sample_rate;
    real minimum_distance;
    real maximum_distance;
    real skip_fraction;
    real pitch_lower_bound;
    real pitch_upper_bound;
    real inner_cone_angle;
    real outer_cone_angle;
    real outer_cone_gain;
    real gain;
    real maximum_bend;
    int32_t unused[3];
    struct sound_scale_modifiers scale_lower_bound;
    struct sound_scale_modifiers scale_upper_bound;
    int16_t encoding;
    int16_t compression;
    struct tag_reference promotion_sound;
    int16_t promotion_count;
    uint16_t pad2;
    int32_t runtime_maximum_play_time;
    int32_t runtime_promotion_counter;
    int32_t runtime_promotion_time;
    int32_t runtime_scripting_time;
    int32_t runtime_scripting_sound_index;
    struct tag_block pitch_ranges;
};
static_assert(sizeof(struct sound_definition) == 164);

/* sound looping */

enum {
    LOOPING_SOUND_DEFINITION_TAG = 0x6C736E64, // 'lsnd'
    LOOPING_SOUND_DEFINITION_VERSION = 3,
    CUSTOM_MUSIC_PLAY_ID = 0x6D706C79, // 'mply'
    MAXIMUM_TRACKS_PER_LOOPING_SOUND = 4,
    MAXIMUM_DETAIL_SOUNDS_PER_LOOPING_SOUND = 32
};

enum{
    _fade_in_at_start_bit,
    _fade_out_at_stop_bit,
    _fade_in_alternate_bit,
    NUMBER_OF_LOOPING_SOUND_TRACK_FLAGS
};

struct looping_sound_track{
    uint32_t flags;
    real gain;
    real fade_in_duration;
    real fade_out_duration;
    int32_t unused[8];
    struct tag_reference start_sound;
    struct tag_reference loop_sound;
    struct tag_reference stop_sound;
    int32_t unused2[8];
    struct tag_reference alternate_loop_sound;
    struct tag_reference alternate_stop_sound;
};
static_assert(sizeof(struct looping_sound_track) == 160);

enum {
    _detail_dont_play_with_alternate_bit,
    _detail_dont_play_without_alternate_bit,
    NUMBER_OF_LOOPING_SOUND_DETAIL_FLAGS
};

struct looping_sound_detail {
    struct tag_reference sound;
    real period_lower_bound;
    real period_upper_bound;
    real gain;
    int32_t flags;
    int32_t unused0[12];
    real theta_lower_bound;
    real theta_upper_bound;
    real phi_lower_bound;
    real phi_upper_bound;
    real distance_lower_bound;
    real distance_upper_bound;
};
static_assert(sizeof(struct looping_sound_detail) == 104);

struct looping_sound_scale_modifiers {
    real detail_period;
    int32_t unused0[2];
};

enum {
    _looping_sound_deafening_bit,
    _looping_sound_fake_impulse_sound_bit,
    _looping_sound_stops_music_bit,
    NUMBER_OF_LOOPING_SOUND_FLAGS
};

struct looping_sound_definition {
    uint32_t flags;
    struct looping_sound_scale_modifiers scale_lower_bound;
    struct looping_sound_scale_modifiers scale_upper_bound;
    int32_t runtime_scripting_sound_index;
    real runtime_maximum_distance;
    int32_t unused[2];
    struct tag_reference continuous_damage_effect;
    struct tag_block tracks;
    struct tag_block details;
};
static_assert(sizeof(struct looping_sound_definition) == 84);

/* sound functions */

static inline struct sound_definition *sound_definition_get(int32_t tag_index) {
    return tag_get(SOUND_DEFINITION_TAG, tag_index);
}

static inline struct sound_pitch_range *sound_definition_get_pitch_range(struct sound_definition *sound, int32_t pitch_range_index) {
    return tag_block_get_element_with_size(&sound->pitch_ranges, pitch_range_index, sizeof(struct sound_pitch_range));
}

static inline struct sound_permutation *sound_pitch_range_get_permutation(struct sound_pitch_range *pitch_range, int32_t permutation_index) {
    return tag_block_get_element_with_size(&pitch_range->permutations, permutation_index, sizeof(struct sound_permutation));
}

/* sound looping functions */

static inline struct looping_sound_definition *looping_sound_definition_get(int32_t tag_index) {
    return tag_get(LOOPING_SOUND_DEFINITION_TAG, tag_index);
}

static inline struct looping_sound_track *looping_sound_definition_get_track(struct looping_sound_definition *sound, int32_t track_index) {
    return tag_block_get_element_with_size(&sound->tracks, track_index, sizeof(struct looping_sound_track));
}

static inline struct looping_sound_detail *looping_sound_definition_get_detail(struct looping_sound_definition *sound, int32_t detail_index) {
    return tag_block_get_element_with_size(&sound->details, detail_index, sizeof(struct looping_sound_detail));
}

#endif
