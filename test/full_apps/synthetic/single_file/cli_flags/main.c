enum FlagKind {
    FLAG_HELP = 1,
    FLAG_VERBOSE = 2,
    FLAG_OUTPUT = 4
};

struct CliOption {
    const char *name;
    enum FlagKind kind;
    unsigned takes_value : 1;
};

static struct CliOption cli_options[] = {
    { "--help", FLAG_HELP, 0u },
    { "--verbose", FLAG_VERBOSE, 0u },
    { "--output", FLAG_OUTPUT, 1u }
};

int find_flag(const char *name)
{
    unsigned i;
    for (i = 0; i < sizeof(cli_options) / sizeof(cli_options[0]); ++i) {
        if (cli_options[i].name[2] == name[2]) {
            return (int)cli_options[i].kind;
        }
    }
    return 0;
}
