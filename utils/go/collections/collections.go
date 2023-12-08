package collections

import (
	"slices"
	"strconv"
)

type number interface {
	int | int64 | float64
}

// Slice functions
func Sum[T number](xs []T) (res T) {
	for _, x := range xs {
		res += x
	}
	return
}

func SumFunc[T any, N number](xs []T, f func(T) N) (res N) {
	for _, x := range xs {
		res += f(x)
	}
	return
}

func CountFunc[T comparable](xs []T, f func(T) bool) (res int) {
	for _, x := range xs {
		if f(x) {
			res++
		}
	}
	return
}

func Filter[T any](xs []T, f func(T) bool) []T {
	res := make([]T, 0, len(xs))
	for _, x := range xs {
		if f(x) {
			res = append(res, x)
		}
	}
	return res
}

func Any[T any](xs []T, f func(T) bool) bool {
	for _, x := range xs {
		if f(x) {
			return true
		}
	}
	return false
}

func Map[S, D any](xs []S, f func(S) D) (result []D) {
	for _, x := range xs {
		result = append(result, f(x))
	}
	return
}

func Reduce[T number](xs []T, init T, f func(T, T) T) T {
	result := init
	for _, x := range xs {
		result = f(result, x)
	}
	return result
}

func ParseInts(xs []string) (result []int) {
	for _, x := range xs {
		n, _ := strconv.Atoi(x)
		result = append(result, n)
	}
	return
}

func SetAppend[T comparable](xs []T, e T) []T {
	if !slices.Contains(xs, e) {
		return append(xs, e)
	}
	return xs
}

// Map functions
func MapContains[K, V comparable](m map[K]V, e V) bool {
	for _, v := range m {
		if v == e {
			return true
		}
	}
	return false
}
