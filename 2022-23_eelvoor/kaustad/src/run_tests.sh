#!/bin/zsh
DATA_PATH=~/Downloads/2022-12-11-ev/kaustad
BIN=./target/debug/kaustad
for N in {0..51}
do
    echo Running test $N
    cat $DATA_PATH/input/input$N.txt | $BIN > target/debug/output.txt
    diff --strip-trailing-cr $DATA_PATH/output/output$N.txt target/debug/output.txt
done
