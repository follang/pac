//! Built-in standard C headers for the builtin preprocessor.
//!
//! These provide platform-correct typedefs without needing the real
//! glibc/musl/system headers. The content uses `__SIZEOF_POINTER__`
//! and `__SIZEOF_LONG__` (already defined by our target macros) to
//! pick the right underlying types for each platform.

use std::collections::HashMap;

/// Return a map of built-in header name -> content.
///
/// These are intercepted by the include resolver before searching
/// the filesystem, so `#include <stdint.h>` works without needing
/// the real system header.
pub fn builtin_headers() -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert("stdint.h".into(), STDINT_H.into());
    headers.insert("stddef.h".into(), STDDEF_H.into());
    headers.insert("stdbool.h".into(), STDBOOL_H.into());
    headers
}

const STDINT_H: &str = r#"
#ifndef _PAC_STDINT_H
#define _PAC_STDINT_H 1

/* Exact-width signed types */
typedef signed char        int8_t;
typedef signed short       int16_t;
typedef signed int         int32_t;

/* Exact-width unsigned types */
typedef unsigned char      uint8_t;
typedef unsigned short     uint16_t;
typedef unsigned int       uint32_t;

/* 64-bit types depend on word size */
#if __SIZEOF_LONG__ == 8
typedef signed long        int64_t;
typedef unsigned long      uint64_t;
#else
typedef signed long long   int64_t;
typedef unsigned long long uint64_t;
#endif

/* Least-width types */
typedef int8_t             int_least8_t;
typedef int16_t            int_least16_t;
typedef int32_t            int_least32_t;
typedef int64_t            int_least64_t;
typedef uint8_t            uint_least8_t;
typedef uint16_t           uint_least16_t;
typedef uint32_t           uint_least32_t;
typedef uint64_t           uint_least64_t;

/* Fast types */
typedef signed char        int_fast8_t;
typedef unsigned char      uint_fast8_t;
#if __SIZEOF_LONG__ == 8
typedef signed long        int_fast16_t;
typedef signed long        int_fast32_t;
typedef signed long        int_fast64_t;
typedef unsigned long      uint_fast16_t;
typedef unsigned long      uint_fast32_t;
typedef unsigned long      uint_fast64_t;
#else
typedef signed int         int_fast16_t;
typedef signed int         int_fast32_t;
typedef signed long long   int_fast64_t;
typedef unsigned int       uint_fast16_t;
typedef unsigned int       uint_fast32_t;
typedef unsigned long long uint_fast64_t;
#endif

/* Pointer-width types */
#if __SIZEOF_POINTER__ == 8
typedef signed long        intptr_t;
typedef unsigned long      uintptr_t;
#else
typedef signed int         intptr_t;
typedef unsigned int       uintptr_t;
#endif

/* Max-width types */
#if __SIZEOF_LONG__ == 8
typedef signed long        intmax_t;
typedef unsigned long      uintmax_t;
#else
typedef signed long long   intmax_t;
typedef unsigned long long uintmax_t;
#endif

#endif /* _PAC_STDINT_H */
"#;

const STDDEF_H: &str = r#"
#ifndef _PAC_STDDEF_H
#define _PAC_STDDEF_H 1

/* size_t and ptrdiff_t depend on pointer width */
#if __SIZEOF_POINTER__ == 8
typedef unsigned long  size_t;
typedef signed long    ptrdiff_t;
#else
typedef unsigned int   size_t;
typedef signed int     ptrdiff_t;
#endif

typedef int            wchar_t;

#define NULL ((void *)0)

#endif /* _PAC_STDDEF_H */
"#;

const STDBOOL_H: &str = r#"
#ifndef _PAC_STDBOOL_H
#define _PAC_STDBOOL_H 1

#define bool  _Bool
#define true  1
#define false 0

#endif /* _PAC_STDBOOL_H */
"#;
