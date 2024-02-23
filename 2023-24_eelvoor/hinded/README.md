# hinded

## Translation

Hind (grades) - 1 sec / 5 sec 60 points

Juku has completed a bunch of tests at school, which were graded from 0 to 100 points.

Juku's grandfather gives him pocket money based on his grades. Grandpa considers scores over 50 points as good grades and scores below 50 points as bad grades. Specifically, he adds the parts of the grades exceeding 50 points to Juku's pocket money and subtracts the parts below 50 points from there. For example, for the grades 35, 42, 81, and 100, Juku would receive a total of (35−50) + (42−50) + (81−50) + (100−50) = −15 − 8 + 31 + 50 = 58 euros. (Works graded exactly 50 points do not affect the pocket money).

The teacher keeps Juku's grades in an Excel table with N rows and N columns. Therefore, Juku has a total of N^2 grades. Juku gets access to the table for a moment and has the opportunity to "improve" his grades by marking one rectangular subarea in the table (which can also be empty, i.e., contain zero elements) and deleting all the grades from there.

Find the maximum amount of pocket money that Juku could receive after such deletion. 

*Input.* The first line of input contains an integer N (2 ⩽ N ⩽ 300): the side length of the teacher's table. The next N lines each contain N integers separated by spaces from the range 0 to 100: Juku's grades.

*Output.* Output a single integer: the maximum sum of pocket money for Juku.

*Example.*

    Input   Output
    3       200
    80 90 90
    100 5 60
    90 60 10

Juku should delete the grades 5, 60, 60, 10 from the bottom right part of the table.

*Example.*

    Input   Output
    4       500
    100 100 100 100
    100 2 2 100
    100 90 90 100
    100 2 2 100

Juku should delete all grades 2 and 90.

*Evaluation.* In this task, tests are divided into groups. Only solutions that pass all the tests in the group receive points. The following additional conditions apply to each group:

1. (10 points) N ⩽ 12.
2. (20 points) N ⩽ 80.
3. (30 points) No additional constraints apply.

## Results

I implemented a solution using a summed-area table.

It's fast, but the response isn't correct for all test cases, not sure why, as my code seems fine.
