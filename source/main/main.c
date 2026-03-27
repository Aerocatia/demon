void main_crash([[maybe_unused]] const char *str) {
    *((char **)nullptr) = "chucky was here!  NULL belongs to me!!!!!";
}
