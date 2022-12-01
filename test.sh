#!/bin/bash

if [[ -z $1 ]]; then
    YEARS=$(ls | grep aoc)
else
    YEARS=$1
fi

# Install OCaml utils
for u in $(ls utils/ocaml); do
    cd utils/ocaml/$u
    opam exec -- dune build
    opam exec -- dune install 2> /dev/null
    cd $OLDPWD
done

status=0
for year in $YEARS; do
    cd $year
    echo ---------
    echo "AoC 20"${year:3:2}
    echo ---------

    BUILD=
    # Determine language
    if [[ ! -z $(find -name "*.rs") ]]; then
        # Rust -> use Cargo
        RUN="cargo run --release -q"
    elif [[ ! -z $(find -name "*.ml") ]]; then
        # OCaml -> use Dune
        BUILD="opam exec -- dune build"
        RUN="_build/default/DAY.exe"
    else
        # Everything else (currently Kotlin) -> use make
        RUN="make run -s"
    fi

    # Run tests (for each day, compare output with expected)
    for day in $(ls | grep day); do
        cd $day
        if [[ ! -z $BUILD ]]; then
            $BUILD
        fi
        echo -n $day:
        diff=$(diff <(${RUN/DAY/$day}) expected)
        if [[ $? -eq 0 ]]; then
            echo OK
        else
            echo NOT OK
            echo $diff
            status=1
        fi
        cd ..
    done

    cd ..
    echo
done

exit $status
