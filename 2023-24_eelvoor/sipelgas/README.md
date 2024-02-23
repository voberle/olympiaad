# sipelgas

## Translation

Ant - 1 second 40 points

The robot ant moves along the edges of a cube. The ant always stops at the top of the cube and waits for a command: upon receiving the command "V," it moves to the next vertex along the edge to its left, and upon receiving the command "P," it moves along the edge to its right.

Write a program that takes the sequence of commands executed by the ant so far and finds the shortest possible path back to the vertex from which it started its movement.

*Input.* The first line of input contains the number N of commands executed by the ant so far (0 ⩽ N ⩽ 1000). The second line contains N letters V and P: the list of these commands.

*Output.* Output on the first line the minimum number of commands required to direct the ant back to the vertex from which it started its movement. Output on the second line the list of commands required for this purpose (as a single word, without spaces or other separators). If there are multiple paths to the starting vertex with the minimum number of commands, output any of them.

*Example.*

    Input   Output
    3       1
    V V V   V

In the figure below, the red arrows indicate the ant's location and head direction, and the blue and green lines indicate movements according to the commands given in the input and output, respectively.

## Results

This exercise just sucks. It's not about programming at all, but about who manages to see things well in 3D. The coding then is trivial. Dumb stuff.

NB: I reimplemented the solution from the responses.