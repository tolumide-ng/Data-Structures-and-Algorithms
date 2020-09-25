function Graph () {
    this.adjList = {}
  }
  
  Graph.prototype.addVertex = function(vertex){
    this.adjList[vertex] = []
  }
  
  Graph.prototype.addEdge = function(vertex1, vertex2) {
    this.adjList[vertex1].push(vertex2)
  }
  
  Graph.prototype.dfs = function(){
      const nodes = Object.keys(this.adjList);
      const visited = {};
  
      for(let i=0; i< nodes.length; i++){
          const node = nodes[i];
          this._dfsUtil(node, visited);
      }
  }
  
  
  Graph.prototype._dfsUtil = function(vertex, visited) {
      if(!visited[vertex]){
          visited[vertex]= true;
          console.log(vertex, visited);
          const neighbours = this.adjList[vertex];
          for(let i=0; i< neighbours.length; i++){
              const neighbour = neighbour[i];
              this._dfsUtil(neighbour, visited);
          }
      }
  }
  
  
  (Graph.prototype.detectCycle = function (){
      const graphNodes = Object.keys(this.adjList);
      const visited = {};
      const recStack = {};
  
      for(let i=0; i< graphNodes.length; i++){
          const node = graphNodes[i];
  
          if(this._detectCycleUtil(node, visited, recStack)){
              return "there is a cycle";
          } else {
              return "no cycle";
          }
      }
  })
  
  
  
  Graph.prototype._detectCycleUtil = function (vertex, visited, recStack) {
      if(!visited[vertex]){
          visited[vertex] = true;
              recStack[vertex] = true;
  
                  const nodeNeigbours = this.adjList[vertex];
  
                  for(let i=0; i< nodeNeigbours.length; i++){
                      const currentNode = nodeNeigbours[i];
                      console.log("parent", vertex, "child", currentNode);
                  if(!visited[currentNode] && this._detectCycleUtil(currentNode, visited, recStack)){
                          return true;
                  } else if(recStack[currentNode]){
                          return true;
                  }
                  }
      }
  
      recStack[vertex] = false;
      return false;
  }