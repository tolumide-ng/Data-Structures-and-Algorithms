## Problem Introduction
An undirected graph is called bipartite if its vertices can be split into two parts such that each edge of the graph joins to vertices from different parts. Bipartite graphs arise naturally in applications where a graph is used to model connections between objects of two different types (say, boys and girls; or students and dormitories).
 An alternative definition is the following: a graph is bipartite if its vertices can be colored with two colors (say, black and white) such that the endpoints of each edge have different colors.


## Problem Description

### Task. => 
  Given an undirected graph with 𝑛 vertices and 𝑚 edges, check whether it is bipartite. 
  
### Input Format. =>
  A graph is given in the standard format.

### Output Format. => 
  Output 1 if the graph is bipartite and 0 otherwise.

> Example:  
> 4 4  
> 1 2  
> 4 1  
> 2 3  
> 3 1  

> Output ===> 0

> Example:  
> 5 4  
> 5 2  
> 4 2  
> 3 4  
> 1 4  

> Ouput: ====> 1