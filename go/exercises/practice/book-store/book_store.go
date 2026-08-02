package bookstore

import (
	"sort"
)

// Discount percentages corresponding to group size (1 to 5)
// Group size 0: 0%
// Group size 1: 0% ($8.00 * 1.0 = 800)
// Group size 2: 5% ($8.00 * 2 * 0.95 = 1520)
// Group size 3: 10% ($8.00 * 3 * 0.90 = 2160)
// Group size 4: 20% ($8.00 * 4 * 0.80 = 2560)
// Group size 5: 25% ($8.00 * 5 * 0.75 = 3000)
var discounts = [6]float64{0.0, 0.0, 0.05, 0.10, 0.20, 0.25}

const bookPrice = 800 // in cents

// Cost calculates the minimum possible cost for the shopping basket of books.
func Cost(books []int) int {
	if len(books) == 0 {
		return 0
	}

	// Count frequencies of each book
	countsMap := make(map[int]int)
	for _, b := range books {
		countsMap[b]++
	}

	// Extract counts into a slice
	var counts []int
	for _, count := range countsMap {
		counts = append(counts, count)
	}

	// Memoization cache for dynamic programming / recursion
	memo := make(map[string]int)

	var minCost func(counts []int) int
	minCost = func(c []int) int {
		// Filter out zeros and sort descending to normalize state
		var nonZero []int
		for _, v := range c {
			if v > 0 {
				nonZero = append(nonZero, v)
			}
		}
		if len(nonZero) == 0 {
			return 0
		}
		sort.Slice(nonZero, func(i, j int) bool {
			return nonZero[i] > nonZero[j]
		})

		// Construct cache key
		key := ""
		for _, v := range nonZero {
			key += string(rune(v + '0'))
		}

		if val, ok := memo[key]; ok {
			return val
		}

		min := 1 << 30

		// Try forming groups of size k from 1 up to max available distinct books (max 5)
		maxGroupSize := len(nonZero)
		if maxGroupSize > 5 {
			maxGroupSize = 5
		}

		for k := 1; k <= maxGroupSize; k++ {
			// Form a group of size k by taking 1 from the top k books
			nextCounts := make([]int, len(nonZero))
			copy(nextCounts, nonZero)
			for i := 0; i < k; i++ {
				nextCounts[i]--
			}

			// Cost of this group
			groupCost := int(float64(k*bookPrice) * (1.0 - discounts[k]))
			total := groupCost + minCost(nextCounts)
			if total < min {
				min = total
			}
		}

		memo[key] = min
		return min
	}

	return minCost(counts)
}
