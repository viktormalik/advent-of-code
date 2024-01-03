package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strings"
)

type Graph map[string][]string

type Edge struct {
	From, To string
}

type Path []string

func shortest(from string, to string, graph Graph, residual map[Edge]int) Path {
	todo := []Path{{from}}
	visited := map[string]bool{}
	for len(todo) > 0 {
		cur := todo[0]
		todo = todo[1:]
		node := cur[len(cur)-1]
		for _, n := range graph[node] {
			if residual[Edge{node, n}] <= 0 || visited[n] {
				continue
			}
			visited[n] = true
			path := make(Path, len(cur), len(cur)+1)
			copy(path, cur)
			path = append(path, n)
			if n == to {
				return path
			} else {
				todo = append(todo, path)
			}
		}
	}
	return Path{}
}

func residualNetwork(source, sink string, graph Graph) (result map[Edge]int) {
	result = map[Edge]int{}
	for node, ns := range graph {
		for _, n := range ns {
			result[Edge{node, n}] = 1
			result[Edge{n, node}] = 1
		}
	}
	for {
		if path := shortest(source, sink, graph, result); len(path) > 0 {
			for i := 0; i < len(path)-1; i++ {
				result[Edge{path[i], path[i+1]}]--
				result[Edge{path[i+1], path[i]}]++
			}
		} else {
			return
		}
	}
}

func reachable(from string, graph Graph, residual map[Edge]int) []string {
	todo := []string{from}
	result := []string{from}
	for len(todo) > 0 {
		cur := todo[0]
		todo = todo[1:]
		for _, n := range graph[cur] {
			if residual[Edge{cur, n}] <= 0 || slices.Contains(result, n) {
				continue
			}
			todo = append(todo, n)
			result = append(result, n)
		}
	}
	return result
}

func stCut(source, sink string, graph Graph) (int, int) {
	cutSize := 0
	group := reachable(source, graph, residualNetwork(source, sink, graph))
	for _, node := range group {
		for _, n := range graph[node] {
			if !slices.Contains(group, n) {
				cutSize++
			}
		}
	}
	return cutSize, len(group)
}

func randomNode(graph Graph) string {
	for node := range graph {
		return node
	}
	return ""
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	graph := Graph{}
	for scanner.Scan() {
		split := strings.Split(scanner.Text(), ": ")
		nodes := strings.Fields(split[1])
		node := split[0]
		graph[node] = append(graph[node], nodes...)
		for _, n := range nodes {
			graph[n] = append(graph[n], node)
		}
	}

	for {
		source := randomNode(graph)
		sink := randomNode(graph)
		cut, group := stCut(source, sink, graph)
		if cut == 3 {
			fmt.Println("First:", group*(len(graph)-group))
			return
		}
	}
}
