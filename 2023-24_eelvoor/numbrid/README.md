# numbrid

## Translation

Numbers

Newly founded football club players choose numbers for their shirts. Each player says which number they want.

Write a program that finds all such numbers that nobody wanted, but for which there exists a player who wanted a smaller number, and a player who wanted a larger number.

*Input.* The first line of input contains the number of players N (1 ⩽ N ⩽ 50,000). The second line contains N integers separated by spaces: the numbers desired by the players. All these numbers are in the range 0...10,000,000.

*Output.* The first line of output should contain the total number M of unwanted numbers. The second line should output these numbers separated by spaces and in ascending order. It can be assumed that for a correct answer M ⩽ 50,000.

Example.

    Input           Output
    7               2
    1 2 3 6 7 8 9   4 5

*Evaluation.* In tests that give the first 10 points, the desired numbers of players in the input are listed in ascending order. In tests that give the remaining 10 points, there are no additional constraints.

## Results

Took me 17 minutes for the version that passes all the tests. However I'm not sure it's fast enough, as for test 20 it takes 730 ms on my M1, a bit too close to the 1 second it's supposed to run on a 1,5 GHz Pentium IV.

*Update:* Switching to a HashSet instead of a Vec for the input made it much faster, down to 5 ms. Took me about an extra 10 mins.
