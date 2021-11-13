package main

import (
	"fmt"
	"log"
	"math"
)

func main() {
	var (
		n    int
		numK int
	)
	fmt.Scanf("%d %d", &n, &numK)
	h_slice := make([]int, n)
	for i := int(0); i < n; i++ {
		fmt.Scanf("%d", &h_slice[i])
		log.Println(h_slice[i])
	}

	cost_dp := make([]int, n)
	cost_dp[0] = 0
	for i := int(1); i < n; i++ {
		for k := int(1); k <= numK; k++ {
			if k == 1 {
				cost_dp[i] = cost_dp[i-k] + int(math.Abs(float64(h_slice[i]-h_slice[i-k])))
			} else if i-k >= 0 {
				cost_dp[i] = min(cost_dp[i], cost_dp[i-k]+int(math.Abs(float64(h_slice[i]-h_slice[i-k]))))
			} else {
				break
			}
		}
	}

	log.Println(cost_dp)
	fmt.Printf("%d\n", cost_dp[n-1])
}

///////////
// utils //
///////////

func min(nums ...int) int {
	if len(nums) == 0 {
		panic("funciton min() requires at least one argument.")
	}
	res := nums[0]
	for i := 0; i < len(nums); i++ {
		res = int(math.Min(float64(res), float64(nums[i])))
	}
	return res
}
