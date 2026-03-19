struct HashEntry {
    const char *key;
    int value;
    unsigned used : 1;
};

struct HashTable {
    struct HashEntry entries[8];
    unsigned count;
};

static unsigned hash_byte(unsigned seed, unsigned byte)
{
    return seed * 33u + byte;
}

static struct HashTable make_table(void)
{
    struct HashTable table = {
        .entries = {
            [0] = { "alpha", 1, 1u },
            [3] = { "beta", 2, 1u }
        },
        .count = 2u
    };
    return table;
}
