package forth

import (
	"errors"
	"strconv"
	"strings"
)

type Evaluator struct {
	stack []int
	words map[string][]string
}

func NewEvaluator() *Evaluator {
	return &Evaluator{
		stack: []int{},
		words: make(map[string][]string),
	}
}

func (e *Evaluator) evaluateTokens(tokens []string) error {
	i := 0
	for i < len(tokens) {
		token := strings.ToLower(tokens[i])
		if token == ":" {
			// Definition: : name body... ;
			if i+1 >= len(tokens) {
				return errors.New("empty definition")
			}
			name := strings.ToLower(tokens[i+1])
			// Check if name is a number
			if _, err := strconv.Atoi(name); err == nil {
				return errors.New("illegal operation")
			}
			i += 2
			bodyStart := i
			for i < len(tokens) && strings.ToLower(tokens[i]) != ";" {
				i++
			}
			if i >= len(tokens) {
				return errors.New("unterminated definition")
			}
			body := tokens[bodyStart:i]

			// Save old definition of name if any
			oldBody, hasOld := e.words[name]

			resolvedBody := e.expandBody(body, name, oldBody, hasOld)
			e.words[name] = resolvedBody

			i++ // skip ";"
		} else {
			if err := e.executeToken(token); err != nil {
				return err
			}
			i++
		}
	}
	return nil
}

func (e *Evaluator) expandBody(body []string, currentName string, oldBody []string, hasOld bool) []string {
	var result []string
	for _, t := range body {
		tLower := strings.ToLower(t)
		if tLower == currentName {
			if hasOld {
				result = append(result, e.expandBody(oldBody, "", nil, false)...)
			} else {
				result = append(result, tLower)
			}
		} else if sub, ok := e.words[tLower]; ok {
			result = append(result, e.expandBody(sub, "", nil, false)...)
		} else {
			result = append(result, tLower)
		}
	}
	return result
}

func (e *Evaluator) executeToken(token string) error {
	// 1. Is it a number?
	if val, err := strconv.Atoi(token); err == nil {
		e.stack = append(e.stack, val)
		return nil
	}

	// 2. Is it a user-defined word?
	if body, ok := e.words[token]; ok {
		for _, bt := range body {
			if err := e.executeToken(bt); err != nil {
				return err
			}
		}
		return nil
	}

	// 3. Built-in operations
	switch token {
	case "+":
		if len(e.stack) < 2 {
			if len(e.stack) == 0 {
				return errors.New("empty stack")
			}
			return errors.New("only one value on the stack")
		}
		b := e.stack[len(e.stack)-1]
		a := e.stack[len(e.stack)-2]
		e.stack = e.stack[:len(e.stack)-2]
		e.stack = append(e.stack, a+b)
	case "-":
		if len(e.stack) < 2 {
			if len(e.stack) == 0 {
				return errors.New("empty stack")
			}
			return errors.New("only one value on the stack")
		}
		b := e.stack[len(e.stack)-1]
		a := e.stack[len(e.stack)-2]
		e.stack = e.stack[:len(e.stack)-2]
		e.stack = append(e.stack, a-b)
	case "*":
		if len(e.stack) < 2 {
			if len(e.stack) == 0 {
				return errors.New("empty stack")
			}
			return errors.New("only one value on the stack")
		}
		b := e.stack[len(e.stack)-1]
		a := e.stack[len(e.stack)-2]
		e.stack = e.stack[:len(e.stack)-2]
		e.stack = append(e.stack, a*b)
	case "/":
		if len(e.stack) < 2 {
			if len(e.stack) == 0 {
				return errors.New("empty stack")
			}
			return errors.New("only one value on the stack")
		}
		b := e.stack[len(e.stack)-1]
		a := e.stack[len(e.stack)-2]
		if b == 0 {
			return errors.New("divide by zero")
		}
		e.stack = e.stack[:len(e.stack)-2]
		e.stack = append(e.stack, a/b)
	case "dup":
		if len(e.stack) < 1 {
			return errors.New("empty stack")
		}
		top := e.stack[len(e.stack)-1]
		e.stack = append(e.stack, top)
	case "drop":
		if len(e.stack) < 1 {
			return errors.New("empty stack")
		}
		e.stack = e.stack[:len(e.stack)-1]
	case "swap":
		if len(e.stack) < 2 {
			if len(e.stack) == 0 {
				return errors.New("empty stack")
			}
			return errors.New("only one value on the stack")
		}
		n := len(e.stack)
		e.stack[n-1], e.stack[n-2] = e.stack[n-2], e.stack[n-1]
	case "over":
		if len(e.stack) < 2 {
			if len(e.stack) == 0 {
				return errors.New("empty stack")
			}
			return errors.New("only one value on the stack")
		}
		second := e.stack[len(e.stack)-2]
		e.stack = append(e.stack, second)
	default:
		return errors.New("undefined operation")
	}

	return nil
}

func Forth(input []string) ([]int, error) {
	eval := NewEvaluator()
	for _, line := range input {
		fields := strings.Fields(line)
		if err := eval.evaluateTokens(fields); err != nil {
			return nil, err
		}
	}
	return eval.stack, nil
}
