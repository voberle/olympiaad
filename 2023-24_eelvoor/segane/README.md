# segane

Exercise 6 of Eesti informaatikaolümpiaad 2023-2024 Eelvoor.

[Original description in Estonian](https://eio.ee/uploads/Main/2023-12-10-ev-et.zip).

## Translation

Confusing Output (confusing) - 3 sec / 10 sec 60 points

Juku experimented with various command-line programs. When running most programs multiple times in a row, he got the same output multiple times. For example, the pwd program produced the following when run twice in a row:

    /home/juku/lahendused/segane
    /home/juku/lahendused/segane

However, such output is guaranteed only when the programs are executed sequentially. When Juku launched multiple instances of one program simultaneously, he noticed that their outputs intertwined, resulting in a different outcome with each attempt. For instance, when running pwd with two instances simultaneously, Juku got the following outputs on two occasions:

    /home//homjukue/ju/lahkeundu/sedl/ahseeganendused/segane
    /h/ohomme/e/juku/lajukhue/landusehedn/segadunseed/segane

Juku noticed that although the characters output by different instances of the program might interleave, the entire output of each instance was complete, and all characters output by one instance were in the correct order.

Juku decided to repeatedly experiment with this strange phenomenon, but after some time trying, he realized that he actually didn't know the output of some programs. Now, based on the existing information, he wants to reconstruct the possible outputs.

*Input.* In this task, the input may consist of multiple subtests. The first line of the input specifies the number of subtests T (1 ⩽ T ⩽ 20).

Each subtest consists of two lines. The first line contains the number of executed programs N (2 ⩽ N ⩽ 10). The second line contains a string S, the output obtained from running N instances of one program simultaneously. For simplicity, this string consists only of lowercase Latin letters and does not contain spaces, line breaks, or other special characters.

*Output.* For each subtest, output two lines. The first line should contain an integer indicating the number of all possible outputs of the program. The second line should contain all possible outputs of the program separated by spaces and in alphabetical order.

Let L be the length of the output of one instance of the program (the length of S is then N · L). For each subtest, it is guaranteed that N ^ L ⩽ 2 · 10^7. The following table shows the maximum values of L corresponding to possible values of N:

    N   max L
    2   24
    3   15
    4   12
    5   10
    6   9
    7   8
    8   7
    9   6
    10  6

*Example.*

    Input                     Output
    4                         1
    2                         abcd
    abcdabcd                  1
    5                         aa
    aaaaaaaaaa                2
    2                         aabb abab
    aabbaabb                  1
    4                         error
    eerrrroereorrrorrror

In the first subtest, the outputs of program instances are output sequentially. In the second subtest, the program instances could output characters in many different ways, but there is only one possible variant for the program output. The third subtest illustrates a situation where the program could output one of two possible outputs.

*Evaluation.* In this task, tests (inputs) are divided into groups. Points are awarded only to those solutions that pass all tests belonging to the group. The following additional conditions apply to the groups:

1. (15 points) N = 2, L ⩽ 11.
2. (15 points) N = 2.
3. (30 points) No additional conditions.

## Scoring

1. (15 points) N = 2, L ⩽ 11.

    ../run_tests.sh 1..3

2. (15 points) N = 2.

    ../run_tests.sh 4..6
    ../run_tests.sh 1..6

3. (30 points) No additional conditions.

    ../run_tests.sh 7..12
    ../run_tests.sh

## Results

Version history:

- Initial version implemented for N = 2 and works for evaluation 1. It worked for part 2 as well (except the "kkk" case in test 4), but it's too slow.
- Optimized it by adding a cache to prevent recursing into stuff we already did. That fixed the performance issue and got it working for part 2.
- Initial version for N > 1. Passes for tests 0 to 7.
