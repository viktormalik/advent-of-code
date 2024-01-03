package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"math"
	"os"
	"strings"
)

type Vec struct {
	X, Y, Z float64
}

type Stone struct {
	Pos, Vel Vec
}

func intersect(s1, s2 Stone) (x, y float64) {
	a1 := s1.Vel.Y / s1.Vel.X
	a2 := s2.Vel.Y / s2.Vel.X
	b1 := s1.Pos.Y - a1*s1.Pos.X
	b2 := s2.Pos.Y - a2*s2.Pos.X

	x = (b1 - b2) / (a2 - a1)
	y = a1*x + b1

	return
}

func inRange(pos float64) bool {
	return pos >= 200000000000000 && pos <= 400000000000000
}

func inFuture(s Stone, x float64) bool {
	if s.Vel.X >= 0 {
		return x >= s.Pos.X
	} else {
		return x < s.Pos.X
	}
}

func divisors(n int) []int {
	result := []int{1}
	for i := 2; i < int(math.Sqrt(float64(n))); i++ {
		if n%i == 0 {
			result = append(result, i)
		}
	}
	return result
}

func gaussian(s1, s2 Stone, vel Vec) Vec {
	// We know the velocity of our stone and just need to find its initial
	// position. To do that, we just need two other stones s1, s2 which will
	// lead to 6 linear equations with 5 unknown variables (x, y, z for the
	// initial position and t1, t2 for the times of hitting s1, s2, resp.):
	//
	// s1.X + t1 * s1.VX = x + t1 * vx
	// s1.Y + t1 * s1.VY = y + t1 * vy
	// s1.Z + t1 * s1.VZ = z + t1 * vz
	// s2.X + t2 * s2.VX = x + t2 * vx
	// s2.Y + t2 * s2.VY = y + t2 * vy
	// s2.Z + t2 * s2.VZ = z + t2 * vz
	//
	// Omitting the last equation (we only need 5), we get the matrix:
	//
	// x  y  z  t1            t2            N
	// 1  0  0  (vx - s1.VX)  0             s1.X
	// 0  1  0  (vy - s1.VY)  0             s1.Y
	// 0  0  1  (vz - s1.VZ)  0             s1.Z
	// 1  0  0  0             (vx - s2.VX)  s2.X
	// 0  1  0  0             (vy - s2.VY)  s2.Y
	//
	// Below is a Gaussian elimination procedure for the given matrix.

	t1_1 := vel.X - s1.Vel.X
	t1_2 := vel.Y - s1.Vel.Y
	t1_3 := vel.Z - s1.Vel.Z
	t2_4 := vel.X - s2.Vel.X
	t2_5 := vel.Y - s2.Vel.Y

	n_1 := s1.Pos.X
	n_2 := s1.Pos.Y
	n_3 := s1.Pos.Z
	n_4 := s2.Pos.X
	n_5 := s2.Pos.Y

	// row 4 = row 4 - row 1
	t1_4 := -t1_1
	n_4 = n_4 - n_1

	// row 5 = row 5 - row 2
	t1_5 := -t1_2
	n_5 = n_5 - n_2

	// row 5 = row 5 - row 4 * (t1_5/t1_4)
	t2_5 = (t2_5*t1_4 - t1_5*t2_4) / t1_4
	n_5 = (n_5*t1_4 - t1_5*n_4) / t1_4

	// matrix is in row echelon form, compute all variables
	t2 := n_5 / t2_5
	t1 := (n_4 - t2_4*t2) / t1_4

	x := n_1 - t1_1*t1
	y := n_2 - t1_2*t1
	z := n_3 - t1_3*t1

	return Vec{x, y, z}
}

func velCandidates(vel, diff float64) []float64 {
	// With constant difference between coordinates, there are only several
	// possibilities of what the velocity in that direction can be such that it
	// ends up being integer. For each integer divisor of the diff, there are
	// three options, based on the order in which the stones are hit and the
	// relative speed of the two stones and our stone.
	result := []float64{}
	for _, d := range divisors(int(diff)) {
		div := float64(d)
		result = append(result, vel+div, div-vel, vel-div)
	}
	return result
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	stones := []Stone{}
	for scanner.Scan() {
		split := strings.Split(scanner.Text(), " @ ")
		pos := collections.ParseFloats(strings.Split(split[0], ", "))
		vel := collections.ParseFloats(strings.Split(split[1], ", "))
		stones = append(
			stones, Stone{Vec{pos[0], pos[1], pos[2]}, Vec{vel[0], vel[1], vel[2]}},
		)
	}

	first := 0
	var candx, candy, candz []float64
	for i := 0; i < len(stones); i++ {
		for j := i + 1; j < len(stones); j++ {
			x, y := intersect(stones[i], stones[j])
			if inRange(x) && inRange(y) &&
				inFuture(stones[i], x) && inFuture(stones[j], x) {
				first++
			}
			// If two stones have the same velocity in some direction, there are
			// only several options for the velocity of our stone in that direction.
			// When we encounter enough such pairs for each direction, we can
			// find the velocity precisely.
			if stones[i].Vel.X == stones[j].Vel.X && (candx == nil || len(candx) > 1) {
				dx := math.Abs(stones[i].Pos.X - stones[j].Pos.X)
				candx = collections.SetIntersection(
					candx, velCandidates(stones[i].Vel.X, dx))
			}
			if stones[i].Vel.Y == stones[j].Vel.Y && (candy == nil || len(candy) > 1) {
				dy := math.Abs(stones[i].Pos.Y - stones[j].Pos.Y)
				candy = collections.SetIntersection(
					candy, velCandidates(stones[i].Vel.Y, dy))
			}
			if stones[i].Vel.Z == stones[j].Vel.Z && (candz == nil || len(candz) > 1) {
				dz := math.Abs(stones[i].Pos.Z - stones[j].Pos.Z)
				candz = collections.SetIntersection(
					candz, velCandidates(stones[i].Vel.Z, dz))
			}
		}
	}
	fmt.Println("First:", first)

	vel := Vec{candx[0], candy[0], candz[0]}
	pos := gaussian(stones[0], stones[1], vel)
	fmt.Println("Second:", int(pos.X)+int(pos.Y)+int(pos.Z))
}
