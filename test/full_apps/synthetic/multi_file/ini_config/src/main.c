#include "ini_config.h"

int ini_key_matches(const IniPair *pair, const char *section, const char *key)
{
    return pair->section[0] == section[0] && pair->key[0] == key[0];
}

int main(void)
{
    IniPair pair = { "net", "port", "8080" };
    return ini_key_matches(&pair, "net", "port") ? 0 : 1;
}
