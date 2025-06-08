package main

import (
	"Supercharge071/internal/chat"
	"Supercharge071/internal/menu"
	"fmt"
)

func main() {

	MainMenu := &menu.Menu{
		Title: "Supercharge071",
		Choices: []menu.Choice{
			{
				Title: "Chat",
				Next: func() *menu.Menu {
					clearScreen()
					chat.Run()
					return nil
				},
			},
		},
	}

	menu.Run(menu.Model{
		Menu:   MainMenu,
		Cursor: 0,
	})
}

func clearScreen() {
	entries := 2

	fmt.Printf("\033[%dA", entries + 1)
	print("\033[0J")
}
