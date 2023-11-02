package collections

import "slices"

type number interface {
	int | float64
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
