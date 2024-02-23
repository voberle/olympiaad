#!/bin/zsh
TASK=teed
DATA_PATH=~/Downloads/2023-12-10-ev

cargo b --release

BIN=./target/release/$TASK
for N in {0..100}
do
    if [ ! -f $DATA_PATH/$TASK/input/input$N.txt ]
    then
        break;
    fi

    echo Running test $N
    cat $DATA_PATH/$TASK/input/input$N.txt | $BIN > target/release/output.txt
    diff --strip-trailing-cr $DATA_PATH/$TASK/output/output$N.txt target/release/output.txt
done
