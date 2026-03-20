use super::macros::{MacroDef, MacroTable};
use super::token::{Token, TokenKind};

/// Target architecture for predefined macros.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetArch {
    X86_64,
    Aarch64,
    X86,
    Arm,
}

/// Target OS for predefined macros.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetOs {
    Linux,
    Darwin,
    Windows,
}

/// Target description for predefined macro generation.
#[derive(Debug, Clone)]
pub struct Target {
    pub arch: TargetArch,
    pub os: TargetOs,
}

impl Target {
    /// Detect the host target.
    pub fn host() -> Self {
        Target {
            arch: Self::detect_arch(),
            os: Self::detect_os(),
        }
    }

    fn detect_arch() -> TargetArch {
        if cfg!(target_arch = "x86_64") {
            TargetArch::X86_64
        } else if cfg!(target_arch = "aarch64") {
            TargetArch::Aarch64
        } else if cfg!(target_arch = "x86") {
            TargetArch::X86
        } else if cfg!(target_arch = "arm") {
            TargetArch::Arm
        } else {
            TargetArch::X86_64 // fallback
        }
    }

    fn detect_os() -> TargetOs {
        if cfg!(target_os = "linux") {
            TargetOs::Linux
        } else if cfg!(target_os = "macos") {
            TargetOs::Darwin
        } else if cfg!(target_os = "windows") {
            TargetOs::Windows
        } else {
            TargetOs::Linux // fallback
        }
    }
}

/// Populate a macro table with predefined macros for a target.
pub fn define_target_macros(table: &mut MacroTable, target: &Target) {
    // Standard C version macros
    define_obj(table, "__STDC__", "1");
    define_obj(table, "__STDC_VERSION__", "201710L"); // C17
    define_obj(table, "__STDC_HOSTED__", "1");

    // Architecture
    match target.arch {
        TargetArch::X86_64 => {
            define_obj(table, "__x86_64__", "1");
            define_obj(table, "__x86_64", "1");
            define_obj(table, "__amd64__", "1");
            define_obj(table, "__amd64", "1");
            define_obj(table, "__LP64__", "1");
            define_obj(table, "__SIZEOF_POINTER__", "8");
            define_obj(table, "__SIZEOF_LONG__", "8");
            define_obj(table, "__SIZEOF_INT__", "4");
            define_obj(table, "__SIZEOF_SHORT__", "2");
        }
        TargetArch::Aarch64 => {
            define_obj(table, "__aarch64__", "1");
            define_obj(table, "__ARM_64BIT_STATE", "1");
            define_obj(table, "__LP64__", "1");
            define_obj(table, "__SIZEOF_POINTER__", "8");
            define_obj(table, "__SIZEOF_LONG__", "8");
            define_obj(table, "__SIZEOF_INT__", "4");
            define_obj(table, "__SIZEOF_SHORT__", "2");
        }
        TargetArch::X86 => {
            define_obj(table, "__i386__", "1");
            define_obj(table, "__i386", "1");
            define_obj(table, "__SIZEOF_POINTER__", "4");
            define_obj(table, "__SIZEOF_LONG__", "4");
            define_obj(table, "__SIZEOF_INT__", "4");
            define_obj(table, "__SIZEOF_SHORT__", "2");
        }
        TargetArch::Arm => {
            define_obj(table, "__arm__", "1");
            define_obj(table, "__ARM_ARCH", "7");
            define_obj(table, "__SIZEOF_POINTER__", "4");
            define_obj(table, "__SIZEOF_LONG__", "4");
            define_obj(table, "__SIZEOF_INT__", "4");
            define_obj(table, "__SIZEOF_SHORT__", "2");
        }
    }

    // OS
    match target.os {
        TargetOs::Linux => {
            define_obj(table, "__linux__", "1");
            define_obj(table, "__linux", "1");
            define_obj(table, "linux", "1");
            define_obj(table, "__unix__", "1");
            define_obj(table, "__unix", "1");
            define_obj(table, "unix", "1");
            define_obj(table, "__gnu_linux__", "1");
            define_obj(table, "__ELF__", "1");
        }
        TargetOs::Darwin => {
            define_obj(table, "__APPLE__", "1");
            define_obj(table, "__MACH__", "1");
            define_obj(table, "__unix__", "1");
        }
        TargetOs::Windows => {
            define_obj(table, "_WIN32", "1");
            define_obj(table, "__WIN32__", "1");
            if target.arch == TargetArch::X86_64 {
                define_obj(table, "_WIN64", "1");
                define_obj(table, "__WIN64__", "1");
            }
        }
    }

    // Common GCC-compatible builtins
    define_obj(table, "__GNUC__", "14");
    define_obj(table, "__GNUC_MINOR__", "0");
    define_obj(table, "__GNUC_PATCHLEVEL__", "0");

    // Type limits
    define_obj(table, "__CHAR_BIT__", "8");
    define_obj(table, "__INT_MAX__", "2147483647");
    define_obj(table, "__LONG_MAX__", "9223372036854775807L");
    define_obj(table, "__LONG_LONG_MAX__", "9223372036854775807LL");

    // Byte order (most common case)
    define_obj(table, "__BYTE_ORDER__", "1234");
    define_obj(table, "__ORDER_LITTLE_ENDIAN__", "1234");
    define_obj(table, "__ORDER_BIG_ENDIAN__", "4321");

    // Common feature macros
    define_obj(table, "__STDC_UTF_16__", "1");
    define_obj(table, "__STDC_UTF_32__", "1");
}

fn define_obj(table: &mut MacroTable, name: &str, value: &str) {
    let body = vec![Token {
        kind: TokenKind::Number,
        text: value.into(),
        offset: 0,
    }];
    table.define(MacroDef {
        name: name.into(),
        params: None,
        is_variadic: false,
        body,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_target_defines_stdc() {
        let mut table = MacroTable::new();
        define_target_macros(&mut table, &Target::host());
        assert!(table.is_defined("__STDC__"));
        assert!(table.is_defined("__STDC_VERSION__"));
    }

    #[test]
    fn x86_64_linux_target() {
        let mut table = MacroTable::new();
        let target = Target {
            arch: TargetArch::X86_64,
            os: TargetOs::Linux,
        };
        define_target_macros(&mut table, &target);
        assert!(table.is_defined("__x86_64__"));
        assert!(table.is_defined("__linux__"));
        assert!(table.is_defined("__LP64__"));
        assert!(table.is_defined("__ELF__"));
    }

    #[test]
    fn aarch64_darwin_target() {
        let mut table = MacroTable::new();
        let target = Target {
            arch: TargetArch::Aarch64,
            os: TargetOs::Darwin,
        };
        define_target_macros(&mut table, &target);
        assert!(table.is_defined("__aarch64__"));
        assert!(table.is_defined("__APPLE__"));
        assert!(table.is_defined("__MACH__"));
        assert!(!table.is_defined("__linux__"));
    }

    #[test]
    fn x86_64_windows_target() {
        let mut table = MacroTable::new();
        let target = Target {
            arch: TargetArch::X86_64,
            os: TargetOs::Windows,
        };
        define_target_macros(&mut table, &target);
        assert!(table.is_defined("_WIN32"));
        assert!(table.is_defined("_WIN64"));
        assert!(!table.is_defined("__linux__"));
    }

    #[test]
    fn gcc_compat_macros() {
        let mut table = MacroTable::new();
        define_target_macros(&mut table, &Target::host());
        assert!(table.is_defined("__GNUC__"));
        assert!(table.is_defined("__CHAR_BIT__"));
        assert!(table.is_defined("__INT_MAX__"));
    }

    #[test]
    fn predefined_works_with_processor() {
        use super::super::processor::{Processor, tokens_to_text};
        use super::super::lexer::Lexer;

        let mut table = MacroTable::new();
        define_target_macros(&mut table, &Target::host());
        let mut proc = Processor::with_macros(table);

        let src = "#ifdef __STDC__\nint standard;\n#endif\n";
        let tokens = Lexer::tokenize(src);
        let out = proc.process(&tokens);
        let text = tokens_to_text(&out.tokens).trim().to_string();
        assert_eq!(text, "int standard;");
    }
}
