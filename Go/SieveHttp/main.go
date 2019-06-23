package main

import (
	"fmt"
)

func main() {
	fmt.Println("Sieve HTTP")

	var k int
	fmt.Print("Enter K: ")
	_, _ = fmt.Scanf("%d", &k)
	fmt.Println()

	fmt.Println(k)

	sieve(k)
}

func sieve(k int) []int {
	var nonPrimes = make([]int, 0)

	for i := 2; i <= k; i++ {
		var counter = 0

		if contains(nonPrimes, i) {
			continue
		}

		for divisor := 1; divisor <= i+1; divisor++ {
			if i%divisor == 0 {
				counter++
			}
		}

		if counter == 2 {
			fmt.Println(i)
		} else {
			var power = i

			for power <= k {
				nonPrimes = append(nonPrimes, power)
				power = power * i
			}
		}
	}
	return nonPrimes
}

func contains(s []int, e int) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}
