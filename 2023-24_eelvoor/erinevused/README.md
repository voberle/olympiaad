# erinevused

## Translation

Differences (differences) 0.5 sec / 3 sec 30 points

Juku's school's math classes participated in the informatics Olympiad. Now the teacher is analyzing the results and wants to know, among other things, in which class the students' level is the most uniform and in which the most uneven.

The teacher finds the difference between the results of two students by subtracting the score of the student who got more points from the score of the student who got fewer points. To measure the unevenness of the class's students' levels, the teacher looks at all pairs of students, adds up the differences in results for all pairs, and divides the sum by the number of pairs. Dividing the sum by the number of pairs is easy because it's always one operation. But calculating the sum of differences is cumbersome, and Juku plans to write a program for this as a creative project.

Juku's ambition is actually broader; he wants to create a student company and offer results analysis not only to schools but also city by city. However, in million-strong cities, there are hundreds of thousands of students, so the program must be efficient enough to handle large datasets. Help Juku write such a program!

*Input.* The first line of input contains the number of students under investigation, N (2 ⩽ N ⩽ 100,000). The second line contains N integers separated by spaces, A1, A2, . . . , AN (0 ⩽ Ai ⩽ 1,000): the students' results.

*Output.* Output on a single line the sum of differences in results given in the input.

*Example.*

    Input   Output
    3       6
    1 4 2

In the class, there are 3 students who earned 1, 4, and 2 points, respectively. It's possible to form three pairs of students. The difference in results between students 1 and 2 is 4 − 1 = 3 points. The difference in results between students 1 and 3 is 2 − 1 = 1 point. The difference in results between students 2 and 3 is 4 − 2 = 2 points. The sum of differences is 3 + 1 + 2 = 6 points.

*Evaluation.* For tests giving the first 10 points, N ⩽ 1,000. For tests giving the next 10 points, the students' results in the input are not in decreasing order (A1 ⩽ A2 ⩽ . . . ⩽ AN). For tests giving the last 10 points, there are no additional conditions.

## Results

I got a version that passes all the tests in 12 minutes.

But at least test 7 and 9 are too slow (1 sec on a M1).

I didn't see how to optimize it, but from the solutions there was indeed a very nice trick that counts the pairs. That one runs on 3 ms.
