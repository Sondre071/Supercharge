package main

import (
	"log"
	"os"
	tea "github.com/charmbracelet/bubbletea"
	ui "supercharge/internal/root"
)

func main() {
	p := tea.NewProgram(ui.New())

	_, err := p.Run();
	
	if err != nil {
		log.Fatal(err)
		os.Exit(1)
	}
}
