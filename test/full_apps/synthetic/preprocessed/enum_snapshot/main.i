# 1 "enum_snapshot.c"
typedef unsigned long size_t;

enum init_state {
    INIT_COLD = 0,
    INIT_WARM = 1,
    INIT_READY = 2
};

struct init_table {
    enum init_state state;
    const char *name;
    size_t budget;
};

static struct init_table init_tables[] = {
    { INIT_COLD, "cold", 8UL },
    { INIT_WARM, "warm", 16UL },
    { INIT_READY, "ready", 32UL }
};

int init_budget(enum init_state state)
{
    return (int)init_tables[state].budget;
}
