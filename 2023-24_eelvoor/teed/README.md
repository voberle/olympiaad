# Juta teekond (teed)

## Translation

Juta's Journey

Juta walked along the roads shown in the diagram. She always moved from left to right, never from right to left. At each intersection, she had two options to move forward, from which she chose one.

[Juta teekond](./juta_teekond.png)

Juta sequentially wrote down the numbers she encountered on her journey.

Write a program that takes three numbers and determines if Juta could have written those numbers in that order.

*Input.* The input consists of three lines, each containing one integer. The first line contains the number that Juta wrote down first. The second line contains the number she wrote down second. The third line contains the number she wrote down third. All numbers in the input are in the range 0 . . . 100.

*Output.* Output a single word: JAH if Juta could have encountered the given numbers in the given order, and EI if it is impossible. Output should be in uppercase.

Example.

    Input   Output
    1       JAH
    8
    5

Numbers 1, 8, and 5 could be the numbers Juta encountered on her journey: for this, Juta had to first move along the upper branch, next along the lower branch, and finally, also along the lower branch.

Example.

    Input   Output
    2       EI
    18
    85

Numbers 2, 18, and 85 cannot be the numbers Juta encountered on her journey.

Note. Note that your program must not output anything other than the answer. For example, the text "Enter the first number" confuses the evaluation program and may result in not getting points for an otherwise correct solution.

## Result

Implemented it in 8 minutes (without rushing), all correct on first try.