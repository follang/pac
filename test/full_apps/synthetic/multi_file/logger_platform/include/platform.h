#ifndef PLATFORM_H
#define PLATFORM_H

typedef enum LogLevel {
    LOG_INFO,
    LOG_WARN,
    LOG_ERROR
} LogLevel;

typedef struct PlatformSink {
    const char *name;
    unsigned color : 1;
} PlatformSink;

#endif
