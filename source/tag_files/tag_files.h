#ifndef DEMON_TAG_FILES_H
#define DEMON_TAG_FILES_H

#include "../cseries/cseries.h"

constexpr char TAG_FILE_DIRECTORY_STRING[] = "\\";
constexpr char TAG_FILE_EXTENSION_STRING[] = ".";

constexpr char TAG_FILE_DIRECTORY_CHARACTER = '\\';
constexpr char TAG_FILE_EXTENSION_CHARACTER = '.';

constexpr size_t TAG_FILE_NAME_LENGTH = 255;

const char *tag_name_strip_path(const char *name);

#endif
