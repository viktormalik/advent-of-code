package main

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func find_key(key, prefix string) int {
	for i := 1; ; i++ {
		hash := md5.Sum([]byte(key + strconv.Itoa(i)))
		str := hex.EncodeToString(hash[:])
		if strings.HasPrefix(str, prefix) {
			return i
		}
	}
	return 0
}

func main() {
	input, _ := os.ReadFile("input")
	key := strings.TrimSpace(string(input))
	fmt.Println("First:", find_key(key, "00000"))
	fmt.Println("Second:", find_key(key, "000000"))
}
