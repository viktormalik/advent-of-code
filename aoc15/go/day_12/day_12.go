package main

import (
	"aoc/collections"
	"encoding/json"
	"fmt"
	"os"
)

func sum(obj interface{}, nored bool) (res float64) {
	switch val := obj.(type) {
	case float64:
		return val
	case []interface{}:
		for _, e := range val {
			res += sum(e, nored)
		}
	case map[string]interface{}:
		if nored && collections.MapContains(val, "red") {
			return 0
		}
		for _, v := range val {
			res += sum(v, nored)
		}
	default:
	}
	return
}

func main() {
	input, _ := os.ReadFile("input")
	var doc []interface{}

	json.Unmarshal(input, &doc)

	fmt.Println("First:", int(sum(doc, false)))
	fmt.Println("Second:", int(sum(doc, true)))
}
