#!/bin/sh
OP=$1
YEAR=$2
DAY=$3
LANG=$4

ROOT=$(git rev-parse --show-toplevel)
DIR=$ROOT/aoc$YEAR/$LANG/day_$(printf "%02d" $DAY)/
SESSION=$(cat $ROOT/.session-cookie)

URL="https://adventofcode.com/20$YEAR/day/$DAY"

[[ -d $DIR ]] || (echo "Missing directory $DIR" && exit 1)

if [[ $OP == "input" ]]; then
    INPUT="$DIR/input"
    [[ -f $INPUT ]] || 
        curl "$URL/input" --cookie "session=$SESSION" > $INPUT
elif [[ $OP == "desc" ]]; then
    DESC="$DIR/desc"
    [[ -f $DESC ]] || 
        curl $URL --cookie "session=$SESSION" |
        pup --pre '.day-desc' |
        w3m -T text/html -dump > $DESC
else
    echo "Usage: fetch.sh { input | desc } YEAR DAY [LANG]"
    exit 1
fi

exit 0
