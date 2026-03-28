#include "../cseries/build_number.h"
#include "../cseries/platform.h"
#include "console.h"

void main_crash([[maybe_unused]] const char *str) {
    *((char **)nullptr) = "chucky was here!  NULL belongs to me!!!!!";
}

void main_print_version() {
    console_printf(false, TARGET " " PLATFORM_NAME_STRING " " BUILD_NUMBER " " __DATE__ " " __TIME__);
}
