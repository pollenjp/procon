package main

import (
	"fmt"
	"log"
	"math"
)

func main() {
	var n int32
	fmt.Scanf("%d", &n)
	h_slice := make([]int32, n)
	for i := int32(0); i < n; i++ {
		fmt.Scanf("%d", &h_slice[i])
		log.Println(h_slice[i])
	}

	cost_dp := make([]int32, n)
	cost_dp[0] = 0
	cost_dp[1] = int32(math.Abs(float64(h_slice[0] - h_slice[1])))
	for i := int32(2); i < n; i++ {
		cost_dp[i] = int32(
			math.Min(
				float64(cost_dp[i-2])+math.Abs(float64(h_slice[i])-float64(h_slice[i-2])),
				float64(cost_dp[i-1])+math.Abs(float64(h_slice[i])-float64(h_slice[i-1]))))
	}

	log.Println(cost_dp)
	fmt.Printf("%d\n", cost_dp[n-1])
}
