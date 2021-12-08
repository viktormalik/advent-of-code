#!/bin/bash

if [[ -z $1 ]]; then
    YEARS=$(ls | grep aoc)
else
    YEARS=$1
fi

status=0
for year in $YEARS; do
    cd $year
    echo ---------
    echo "AoC 20"${year:3:2}
    echo ---------

    # Determine language
    if [[ ! -z $(find -name "*.rs") ]]; then
        # Rust -> use Cargo
        RUN="cargo run --release -q"
    else
        # Everything else (currently Kotlin) -> use make
        RUN="make run -s"
    fi

    # Run tests (for each day, compare output with expected)
    for day in $(ls | grep day); do
        cd $day
        echo -n $day:
        diff=$(diff <($RUN) expected)
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
