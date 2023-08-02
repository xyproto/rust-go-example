package main

import "C"

//export Add
func Add(x, y int) int {
	return x + y
}

func main() {}
