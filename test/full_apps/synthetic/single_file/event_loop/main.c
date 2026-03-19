typedef int (*EventHandler)(int token, void *ctx);

struct EventSlot {
    int token;
    EventHandler handler;
    void *ctx;
};

static int on_tick(int token, void *ctx)
{
    return token + (ctx != (void *)0);
}

static int on_stop(int token, void *ctx)
{
    return token - (ctx != (void *)0);
}

static struct EventSlot slots[] = {
    { 1, on_tick, (void *)0 },
    { 7, on_stop, (void *)0 }
};

int dispatch_slot(unsigned index)
{
    return slots[index].handler(slots[index].token, slots[index].ctx);
}
