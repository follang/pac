#ifndef TASK_SCHEDULER_H
#define TASK_SCHEDULER_H

#include "task_types.h"

unsigned scheduler_ready(const ScheduledTask *tasks, unsigned len);

#endif
