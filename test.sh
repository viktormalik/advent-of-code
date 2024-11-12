#!/bin/bash

root=$PWD

if [[ -z $1 ]]; then
    YEARS=$(ls | grep aoc)
else
    YEARS=$1
fi

if [[ -n $2 ]]; then
    LANGS=$2
fi

declare -A lang_names=(
    ["rust"]="Rust"
    ["ocaml"]="OCaml"
    ["kotlin"]="Kotlin"
    ["go"]="Go"
    ["nim"]="Nim"
)

get_lang() {
    if [[ ! -z $(find -name "*.rs") ]]; then
        lang="rust"
    elif [[ ! -z $(find -name "*.ml") ]]; then
        lang="ocaml"
    elif [[ ! -z $(find -name "*.kt") ]]; then
        lang="kotlin"
    elif [[ ! -z $(find -name "*.go") ]]; then
        lang="go"
    elif [[ ! -z $(find -name "*.nim") ]]; then
        lang="nim"
    else
        lang="unknown"
    fi
}

run_rust() {
    cargo run --release -q
}

install_ocaml() {
    for u in $(ls $root/utils/ocaml); do
        cd $root/utils/ocaml/$u
        opam exec -- dune build
        opam exec -- dune install 2> /dev/null
        cd $OLDPWD
    done
}

build_ocaml() {
    opam exec -- dune build
}

run_ocaml() {
    _build/default/$1.exe
}

run_kotlin() {
    make run -s
}

run_go() {
    go run .
}

run_nim() {
    nim compile --verbosity:0 -d:release --run $1.nim
}

run_year() {
    year=$1

    if [[ -z $2 ]]; then
        get_lang
    else
        lang=$2
    fi

    echo -----------------
    echo "AoC 20${year:3:2} (${lang_names[$lang]})"
    echo -----------------

    if [[ $(type -t install_$lang) == function ]]; then
        install_$lang
    fi

    # Run tests (for each day, compare output with expected)
    for day in $(ls | grep day); do
        cd $day
        if [[ $(type -t build_$lang) == function ]]; then
            build_$lang
        fi
        echo -n $day:
        diff=$(diff <(run_$lang $day) expected)
        if [[ $? -eq 0 ]]; then
            echo OK
        else
            echo NOT OK
            echo $diff
            status=1
        fi
        cd ..
    done
}

status=0
for year in $YEARS; do
    cd $year

    if $(ls day* > /dev/null 2>&1); then
        run_year $year
    else
        if [[ -z $LANGS ]]; then
            langs=$(ls)
        else
            langs=$(ls | grep $LANGS)
        fi
        for lang in $langs; do
            cd $lang
            run_year $year $lang
            cd ..
        done
    fi

    cd ..
    echo
done

exit $status
