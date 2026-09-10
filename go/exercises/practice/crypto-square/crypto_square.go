package cryptosquare

import (
	"strings"
	"unicode"
)

// Encode encodes a given plaintext using the classic square code method.
//
// Steps:
// 1. Normalize: remove spaces and punctuation, convert letters to lowercase.
// 2. Determine rectangle dimensions: find smallest c and r such that r * c >= L, c >= r, and c - r <= 1.
// 3. Arrange characters in an r x c grid, padding remaining cells with spaces.
// 4. Read columns from left to right, chunking each column of length r separated by spaces.
func Encode(pt string) string {
	var norm []rune
	for _, r := range pt {
		if unicode.IsLetter(r) || unicode.IsDigit(r) {
			norm = append(norm, unicode.ToLower(r))
		}
	}

	length := len(norm)
	if length == 0 {
		return ""
	}

	// Calculate rectangle dimensions:
	// Find smallest integer c such that c * c >= length.
	c := 1
	for c*c < length {
		c++
	}

	// r is either c - 1 or c, satisfying r * c >= length and c - r <= 1.
	r := c
	if (c-1)*c >= length {
		r = c - 1
	}

	// Construct encoded message by reading columns of height r.
	// There are c chunks of length r, separated by spaces.
	var builder strings.Builder
	builder.Grow(c*r + (c - 1))

	for col := 0; col < c; col++ {
		if col > 0 {
			builder.WriteByte(' ')
		}
		for row := 0; row < r; row++ {
			idx := row*c + col
			if idx < length {
				builder.WriteRune(norm[idx])
			} else {
				builder.WriteByte(' ')
			}
		}
	}

	return builder.String()
}
