package main

import (
	"Supercharge071/internal/menu"
	"Supercharge071/internal/openrouter"
	"Supercharge071/internal/chat"
)

func main() {

	MainMenu := &menu.Menu{
		Title: "Supercharge071",
		Choices: []menu.Choice{
			{
				Title: "OpenRouter",
				Next:  chat.Run,
			},
		},
	}

	menu.Run(menu.Model{
		Menu:   MainMenu,
		Cursor: 0,
	})
}
