#include "logger.h"

int logger_enabled(LogLevel level, const PlatformSink *sink)
{
    if (level == LOG_ERROR) {
        return 1;
    }
    return sink->color != 0u;
}

int main(void)
{
    PlatformSink sink = { "stderr", 1u };
    return logger_enabled(LOG_INFO, &sink) ? 0 : 1;
}
