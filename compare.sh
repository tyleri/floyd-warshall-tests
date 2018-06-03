#!/bin/bash

if (( $# < 1 )); then
    echo "Test case number required"
    exit 1
fi

# first re-compile everything
rm -f go.out floyd.native rust.out *.class
go build -o go.out floyd.go
ocamlbuild -pkg Str floyd.native > /dev/null 2>&1
javac FloydJava.java
rustc -O -o rust.out floyd.rs
scalac FloydScala.sc


# then run
TEST_CASE=$1

commands=(
    "python3 floyd.py"
    "pypy3 floyd.py"
    "python2 floyd_py2.py"
    "pypy floyd_py2.py"
    "./go.out"
    "./floyd.native"
    "java FloydJava"
    "luajit floyd.lua"
    "./rust.out"
    "scala FloydScala"
)

for ix in ${!commands[*]}
do
    COMMAND="${commands[$ix]}"
    echo "$COMMAND"

    time cat test_cases/input0$TEST_CASE.txt | $COMMAND | cmp test_cases/output0$TEST_CASE.txt -

    if (( $? == 1 )); then
        echo "$COMMAND failed!"
    fi
    
    echo
done
