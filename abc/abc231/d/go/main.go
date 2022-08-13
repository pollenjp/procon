package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"

	"github.com/emirpasic/gods/sets/hashset"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)

	var inN int
	var inM int
	var err error
	scanner.Scan()
	if inN, err = strconv.Atoi(scanner.Text()); err != nil {
		panic(err)
	}
	scanner.Scan()
	if inM, err =strconv.Atoi(scanner.Text()); err != nil {
		panic(err)
	}

	graph := make([][]int, inN)

	inAB := make([][2]int, inM)
	for i := 0; i < inM; i++ {
		scanner.Scan()
		if inAB[i][0], err = strconv.Atoi(scanner.Text()); err != nil {
			panic(err)
		}
		scanner.Scan()
		if inAB[i][1], err = strconv.Atoi(scanner.Text()); err != nil {
			panic(err)
		}

		inAB[i][0]--
		inAB[i][1]--

		graph[inAB[i][0]] = append(graph[inAB[i][0]], inAB[i][1])
		graph[inAB[i][1]] = append(graph[inAB[i][1]], inAB[i][0])
	}

	// check dim
	for _, neighbors := range graph {
		if len(neighbors) > 2 {
			fmt.Println("No")
			return
		}
	}


	// detect cycle
	unvisited := hashset.New()
	for i := 0; i < inN; i++ {
		unvisited.Add(i)
	}
	for i := 0; i < inN; i++ {
		if !unvisited.Contains(i){
			continue
		}
		visited := hashset.New()
		if detect_cycle(i, -1, &graph, visited) {
			fmt.Println("No")
			return
		}

		for _, v := range visited.Values() {
			unvisited.Remove(v)
		}
	}
	fmt.Println("Yes")

}

func detect_cycle(node int, parent int, graph* [][]int, visited *hashset.Set) bool {
	visited.Add(node)

	for _, adj_node := range (*graph)[node] {
		if adj_node == parent {
			// 多重辺は無い
			continue
		}

		if visited.Contains(adj_node) {
			return true
		}

		if detect_cycle(adj_node, node, graph, visited) {
			return true
		}
	}
	return false
}
