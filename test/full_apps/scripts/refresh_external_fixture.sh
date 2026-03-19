#!/bin/sh
set -eu

usage() {
    cat <<'EOF'
Usage:
  refresh_external_fixture.sh list
  refresh_external_fixture.sh show <fixture>

Known fixtures:
  libpng-header
  musl-stdint
  zlib-adler32
  zlib-header
  zlib-zpipe
EOF
}

show_libpng_header() {
    cat <<'EOF'
fixture=libpng-header
project=libpng
version=v1.6.43
license=libpng-2.0
upstream=https://github.com/pnggroup/libpng
files=png.h;pngconf.h;scripts/pnglibconf.h.prebuilt;LICENSE
target=test/full_apps/external/libpng/header
EOF
}

show_musl_stdint() {
    cat <<'EOF'
fixture=musl-stdint
project=musl
version=v1.2.5
license=MIT
upstream=https://git.musl-libc.org/cgit/musl/
files=include/stdint.h;arch/x86_64/bits/stdint.h;arch/x86_64/bits/alltypes.h.in;include/alltypes.h.in;COPYRIGHT
target=test/full_apps/external/musl/stdint
EOF
}

show_zlib_adler32() {
    cat <<'EOF'
fixture=zlib-adler32
project=zlib
version=v1.3.1
license=Zlib
upstream=https://github.com/madler/zlib
files=adler32.c;zutil.h;zlib.h;zconf.h
target=test/full_apps/external/zlib/adler32_impl
EOF
}

show_zlib_header() {
    cat <<'EOF'
fixture=zlib-header
project=zlib
version=v1.3.1
license=Zlib
upstream=https://github.com/madler/zlib
files=zlib.h;zconf.h;LICENSE
target=test/full_apps/external/zlib/header
EOF
}

show_zlib_zpipe() {
    cat <<'EOF'
fixture=zlib-zpipe
project=zlib
version=v1.3.1
license=public-domain
upstream=https://github.com/madler/zlib
files=examples/zpipe.c;zlib.h;zconf.h
target=test/full_apps/external/zlib/zpipe_example
EOF
}

if [ "$#" -lt 1 ]; then
    usage
    exit 1
fi

case "$1" in
    list)
        echo libpng-header
        echo musl-stdint
        echo zlib-adler32
        echo zlib-header
        echo zlib-zpipe
        ;;
    show)
        if [ "$#" -ne 2 ]; then
            usage
            exit 1
        fi
        case "$2" in
            libpng-header)
                show_libpng_header
                ;;
            musl-stdint)
                show_musl_stdint
                ;;
            zlib-adler32)
                show_zlib_adler32
                ;;
            zlib-header)
                show_zlib_header
                ;;
            zlib-zpipe)
                show_zlib_zpipe
                ;;
            *)
                echo "unknown fixture: $2" >&2
                exit 1
                ;;
        esac
        ;;
    *)
        usage
        exit 1
        ;;
esac
