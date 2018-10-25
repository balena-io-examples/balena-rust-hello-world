#!/bin/sh

set -u

UNAME_ORIG_PATH='/bin/uname-orig'
UNAME_PATH=$(which uname)

main() {
    case "$1" in
        rpi)
            modify armv6l
            ;;
        armv7hf)
            modify armv7l
            ;;
        *)
            echo 'uname not modified'
            ;;
    esac
}

modify() {
    cp $UNAME_PATH $UNAME_ORIG_PATH

    cat << EOF > $UNAME_PATH
#!/bin/bash

if [[ \$1 == "-m" ]]; then
    echo "$1"
else
    $UNAME_ORIG_PATH \$@
fi
EOF

    echo 'uname modified'
}

main "$@" || exit 1
