#ifndef TEST_CASE_H
#define TEST_CASE_H

typedef int (*TestFn)(void);

typedef struct TestCase {
    const char *name;
    TestFn fn;
} TestCase;

#endif
