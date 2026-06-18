package alphametics

import (
	"fmt"
	"sort"
	"strings"
)

// solver holds the state for the backtracking solver.
type solver struct {
	n                     int
	coeffs                []int
	leading               []bool
	sortedRemainingCoeffs [][]int
	used                  [10]bool
	assignment            [10]int
}

// Solve parses the alphametics puzzle and returns the digit assignment for each letter,
// or an error if no solution exists.
func Solve(puzzle string) (map[string]int, error) {
	puzzle = strings.ToUpper(puzzle)

	idx := strings.Index(puzzle, "=")
	if idx == -1 {
		return nil, fmt.Errorf("no '=' found in puzzle")
	}

	lhsStr := puzzle[:idx]
	rhsStr := puzzle[idx:]
	rhsStr = strings.TrimLeft(rhsStr, "= ")

	lhsWords := extractWords(lhsStr)
	rhsWords := extractWords(rhsStr)

	if len(lhsWords) == 0 || len(rhsWords) == 0 {
		return nil, fmt.Errorf("invalid puzzle format")
	}

	coeffsMap := make(map[rune]int)
	leadingLetters := make(map[rune]bool)

	for _, word := range lhsWords {
		if len(word) > 1 {
			leadingLetters[rune(word[0])] = true
		}
		power := 1
		for i := len(word) - 1; i >= 0; i-- {
			r := rune(word[i])
			coeffsMap[r] += power
			power *= 10
		}
	}

	for _, word := range rhsWords {
		if len(word) > 1 {
			leadingLetters[rune(word[0])] = true
		}
		power := 1
		for i := len(word) - 1; i >= 0; i-- {
			r := rune(word[i])
			coeffsMap[r] -= power
			power *= 10
		}
	}

	var letters []rune
	for r := range coeffsMap {
		letters = append(letters, r)
	}

	if len(letters) == 0 {
		return nil, fmt.Errorf("no letters found in puzzle")
	}
	if len(letters) > 10 {
		return nil, fmt.Errorf("more than 10 unique letters")
	}

	// Sort letters by descending absolute coefficient value to optimize search order
	sort.Slice(letters, func(i, j int) bool {
		return abs(coeffsMap[letters[i]]) > abs(coeffsMap[letters[j]])
	})

	n := len(letters)
	coeffs := make([]int, n)
	leading := make([]bool, n)
	for i, r := range letters {
		coeffs[i] = coeffsMap[r]
		leading[i] = leadingLetters[r]
	}

	// Pre-calculate sorted remaining coefficients for pruning
	sortedRemainingCoeffs := make([][]int, n)
	for step := 0; step < n; step++ {
		rem := make([]int, n-step)
		copy(rem, coeffs[step:])
		sortInts(rem)
		sortedRemainingCoeffs[step] = rem
	}

	s := &solver{
		n:                     n,
		coeffs:                coeffs,
		leading:               leading,
		sortedRemainingCoeffs: sortedRemainingCoeffs,
	}

	if !s.backtrack(0, 0) {
		return nil, fmt.Errorf("no solution")
	}

	result := make(map[string]int)
	for i, r := range letters {
		result[string(r)] = s.assignment[i]
	}

	return result, nil
}

// backtrack recursively assigns digits to letters using min-max pruning.
func (s *solver) backtrack(step int, currentSum int) bool {
	if step == s.n {
		return currentSum == 0
	}

	remainingCount := s.n - step
	if remainingCount > 0 {
		remCoeffs := s.sortedRemainingCoeffs[step]

		var availBuf [10]int
		availCount := 0
		for d := 0; d < 10; d++ {
			if !s.used[d] {
				availBuf[availCount] = d
				availCount++
			}
		}
		available := availBuf[:availCount]

		minVal, maxVal := getMinMax(remCoeffs, available)
		target := -currentSum
		if target < minVal || target > maxVal {
			return false // Prune branch
		}
	}

	isLeading := s.leading[step]
	coeff := s.coeffs[step]

	for d := 0; d < 10; d++ {
		if s.used[d] {
			continue
		}
		if d == 0 && isLeading {
			continue
		}

		s.used[d] = true
		s.assignment[step] = d

		if s.backtrack(step+1, currentSum+d*coeff) {
			return true
		}

		s.used[d] = false
	}

	return false
}

// getMinMax calculates the mathematically exact minimum and maximum possible sum
// of remaining coefficients and available digits using the Rearrangement Inequality.
func getMinMax(coeffs []int, availableDigits []int) (int, int) {
	m := len(coeffs)
	k := len(availableDigits)

	p := 0
	for p < m && coeffs[p] < 0 {
		p++
	}

	maxSum := 0
	for i := 0; i < p; i++ {
		maxSum += coeffs[i] * availableDigits[i]
	}
	for j := 0; j < m-p; j++ {
		maxSum += coeffs[p+j] * availableDigits[k-(m-p)+j]
	}

	minSum := 0
	for i := 0; i < p; i++ {
		minSum += coeffs[i] * availableDigits[k-1-i]
	}
	for j := 0; j < m-p; j++ {
		minSum += coeffs[p+j] * availableDigits[m-p-1-j]
	}

	return minSum, maxSum
}

// extractWords returns all contiguous sequences of alphabetic characters.
func extractWords(s string) []string {
	var words []string
	var current []rune
	for _, r := range s {
		if r >= 'A' && r <= 'Z' {
			current = append(current, r)
		} else {
			if len(current) > 0 {
				words = append(words, string(current))
				current = nil
			}
		}
	}
	if len(current) > 0 {
		words = append(words, string(current))
	}
	return words
}

// abs returns the absolute value of x.
func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

// sortInts sorts a slice of ints in ascending order using insertion sort.
func sortInts(a []int) {
	for i := 1; i < len(a); i++ {
		key := a[i]
		j := i - 1
		for j >= 0 && a[j] > key {
			a[j+1] = a[j]
			j--
		}
		a[j+1] = key
	}
}
