package beer

import (
	"errors"
	"fmt"
	"strings"
)

// Song returns the entire 99 bottles of beer song.
func Song() string {
	song, _ := Verses(99, 0)
	return song
}

// Verses returns the verses of the song from start down to stop (inclusive).
func Verses(start, stop int) (string, error) {
	if start < 0 || start > 99 || stop < 0 || stop > 99 || start < stop {
		return "", errors.New("invalid verse range")
	}

	var builder strings.Builder
	for i := start; i >= stop; i-- {
		verse, err := Verse(i)
		if err != nil {
			return "", err
		}
		builder.WriteString(verse)
		builder.WriteString("\n")
	}

	return builder.String(), nil
}

// Verse returns a single verse for the given bottle number n.
func Verse(n int) (string, error) {
	if n < 0 || n > 99 {
		return "", fmt.Errorf("invalid verse number: %d", n)
	}

	switch n {
	case 0:
		return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n", nil
	case 1:
		return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", nil
	case 2:
		return "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n", nil
	default:
		return fmt.Sprintf("%d bottles of beer on the wall, %d bottles of beer.\nTake one down and pass it around, %d bottles of beer on the wall.\n", n, n, n-1), nil
	}
}

