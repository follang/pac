typedef int (*CommandFn)(int code, void *ctx);

typedef struct Command {
    const char *name;
    CommandFn fn;
    unsigned enabled : 1;
    unsigned admin_only : 1;
} Command;

static int handle_ping(int code, void *ctx) {
    return code + (ctx != (void *)0);
}

static int handle_shutdown(int code, void *ctx) {
    return code - (ctx == (void *)0);
}

static const Command COMMANDS[] = {
    [0] = { "ping", handle_ping, 1u, 0u },
    [1] = { "shutdown", handle_shutdown, 1u, 1u }
};

static int dispatch_command(const Command *command, int code, void *ctx) {
    if (!command->enabled) {
        return -1;
    }
    return command->fn(code, ctx);
}

int main(void) {
    return dispatch_command(&COMMANDS[0], 7, (void *)0);
}
