# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
        windows)
            stage=$(mktemp -d)
            ;;
        
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --bin gml_fmt --target $TARGET --release

    cp target/$TARGET/release/gml_fmt $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
