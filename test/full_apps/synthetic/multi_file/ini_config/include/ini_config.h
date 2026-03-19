#ifndef INI_CONFIG_H
#define INI_CONFIG_H

#include "config_types.h"

int ini_key_matches(const IniPair *pair, const char *section, const char *key);

#endif
