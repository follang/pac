#include "task_scheduler.h"

unsigned scheduler_ready(const ScheduledTask *tasks, unsigned len)
{
    unsigned i;
    unsigned ready = 0u;
    for (i = 0; i < len; ++i) {
        ready += tasks[i].run != (TaskFn)0;
    }
    return ready;
}

int main(void)
{
    ScheduledTask tasks[] = {
        { "tick", (TaskFn)0, (void *)0 },
        { "flush", (TaskFn)0, (void *)0 }
    };
    return (int)scheduler_ready(tasks, 2u);
}
