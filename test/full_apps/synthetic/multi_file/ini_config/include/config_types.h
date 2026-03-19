#ifndef CONFIG_TYPES_H
#define CONFIG_TYPES_H

typedef struct IniPair {
    const char *section;
    const char *key;
    const char *value;
} IniPair;

#endif
