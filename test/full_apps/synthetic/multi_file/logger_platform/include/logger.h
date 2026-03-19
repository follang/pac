#ifndef LOGGER_H
#define LOGGER_H

#include "platform.h"

int logger_enabled(LogLevel level, const PlatformSink *sink);

#endif
