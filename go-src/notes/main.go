package main

import (
	"fmt"
)

func main() {
	selected, err := runList()

	if err != nil {
		fmt.Printf("Error (list): %v\n)", err)
	}
	if selected == nil {
		return
	}

	/* 	if err := runView(*selected); err != nil {
		fmt.Printf("Error (view): %v\n", err)
		os.Exit(1)
	} */
}
