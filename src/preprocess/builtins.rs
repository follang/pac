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
    headers.insert("limits.h".into(), LIMITS_H.into());
    headers.insert("stdarg.h".into(), STDARG_H.into());
    headers.insert("sys/types.h".into(), SYS_TYPES_H.into());
    headers.insert("unistd.h".into(), UNISTD_H.into());
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

const LIMITS_H: &str = r#"
#ifndef _PAC_LIMITS_H
#define _PAC_LIMITS_H 1

#define CHAR_BIT __CHAR_BIT__

#define SCHAR_MAX 127
#define SCHAR_MIN (-128)
#define UCHAR_MAX 255

#define SHRT_MAX 32767
#define SHRT_MIN (-32768)
#define USHRT_MAX 65535

#define INT_MAX __INT_MAX__
#define INT_MIN (-__INT_MAX__ - 1)
#define UINT_MAX 4294967295U

#define LONG_MAX __LONG_MAX__
#define LONG_MIN (-__LONG_MAX__ - 1L)
#if __SIZEOF_LONG__ == 8
#  define ULONG_MAX 18446744073709551615UL
#else
#  define ULONG_MAX 4294967295UL
#endif

#define LLONG_MAX __LONG_LONG_MAX__
#define LLONG_MIN (-__LONG_LONG_MAX__ - 1LL)
#define ULLONG_MAX 18446744073709551615ULL

#endif /* _PAC_LIMITS_H */
"#;

const STDARG_H: &str = r#"
#ifndef _PAC_STDARG_H
#define _PAC_STDARG_H 1

typedef char *va_list;

#define va_start(ap, last) ((void)0)
#define va_arg(ap, type) (*(type *)0)
#define va_end(ap) ((void)0)
#define va_copy(dest, src) ((void)0)

#endif /* _PAC_STDARG_H */
"#;

const SYS_TYPES_H: &str = r#"
#ifndef _PAC_SYS_TYPES_H
#define _PAC_SYS_TYPES_H 1

#include <stddef.h>
#include <stdint.h>

#if __SIZEOF_LONG__ == 8
typedef long off_t;
typedef long ssize_t;
#else
typedef long long off_t;
typedef int ssize_t;
#endif

#endif /* _PAC_SYS_TYPES_H */
"#;

const UNISTD_H: &str = r#"
#ifndef _PAC_UNISTD_H
#define _PAC_UNISTD_H 1

#include <sys/types.h>

#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2

#endif /* _PAC_UNISTD_H */
"#;

#[cfg(test)]
mod tests {
    use super::builtin_headers;

    #[test]
    fn builtin_headers_include_limits() {
        let headers = builtin_headers();
        assert!(headers.contains_key("limits.h"));
        let limits = headers.get("limits.h").unwrap();
        assert!(limits.contains("CHAR_BIT"));
        assert!(limits.contains("INT_MAX"));
        assert!(limits.contains("ULONG_MAX"));
    }

    #[test]
    fn builtin_headers_include_zlib_baseline_headers() {
        let headers = builtin_headers();
        let stdarg = headers.get("stdarg.h").expect("stdarg.h builtin");
        let sys_types = headers.get("sys/types.h").expect("sys/types.h builtin");
        let unistd = headers.get("unistd.h").expect("unistd.h builtin");
        assert!(stdarg.contains("va_list"));
        assert!(sys_types.contains("off_t"));
        assert!(unistd.contains("SEEK_SET"));
        assert!(unistd.contains("#include <sys/types.h>"));
    }
}
