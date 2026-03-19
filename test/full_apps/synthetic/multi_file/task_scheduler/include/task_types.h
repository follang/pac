#ifndef TASK_TYPES_H
#define TASK_TYPES_H

typedef void (*TaskFn)(void *ctx);

typedef struct ScheduledTask {
    const char *name;
    TaskFn run;
    void *ctx;
} ScheduledTask;

#endif
