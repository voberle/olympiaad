#!/bin/zsh
#
# Executes tests for this exercise.
# To check the output, it diffs the program output with the expected one.
# So nothing on the screen means the test passes.
#
# Run with no arguments to run all tests.
# To run only a subset of tests:
# Run test 1 and 2:
#   ../run_tests 1..2
# Run test 1:
#   ../run_tests 1..1

# Location of the test files
DATA_PATH=~/Downloads/2023-12-10-ev

TASK="$(basename $PWD)"
BIN=./target/release/$TASK

if [ -z "$1" ]
then
    RANGE=0..100
else
    RANGE=$1
fi

cargo b --release

for N in {$RANGE}
do
    if [ ! -f $DATA_PATH/$TASK/input/input$N.txt ]
    then
        break;
    fi

    echo "Running test $N ($DATA_PATH/$TASK/input/input$N.txt)"
    cat $DATA_PATH/$TASK/input/input$N.txt | $BIN > target/release/output.txt 2>/dev/null
    diff --strip-trailing-cr $DATA_PATH/$TASK/output/output$N.txt target/release/output.txt

    # hyperfine "cat $DATA_PATH/$TASK/input/input$N.txt | $BIN > target/release/output.txt"
done
