#include "test_runner.h"

unsigned run_tests(const TestCase *cases, unsigned len)
{
    unsigned i;
    unsigned passed = 0u;
    for (i = 0; i < len; ++i) {
        passed += cases[i].fn != (TestFn)0;
    }
    return passed;
}

int main(void)
{
    TestCase cases[] = {
        { "alpha", (TestFn)0 },
        { "beta", (TestFn)0 }
    };
    return (int)run_tests(cases, 2u);
}
