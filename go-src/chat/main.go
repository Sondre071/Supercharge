package main

import (
	"fmt"
	"os"
	//"time"

	//"github.com/charmbracelet/bubbles/spinner"
	//"github.com/charmbracelet/bubbles/textarea"
	//"github.com/charmbracelet/bubbles/viewport"
	//"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	//"github.com/charmbracelet/lipgloss"
)

type message struct {
	role    string
	content string
}

type model struct {
	messages []message
	input    string
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {

	case tea.KeyMsg:
		switch msg.String() {

		case "enter":
			m.messages = append(m.messages, message{role: "user", content: m.input})
			m.input = ""

		case "ctrl+c", "q":
			return m, tea.Quit

		default:
			m.input += msg.String()
		}
	}

	return m, nil
}

func (m model) View() string {
	s := ""

	for _, msg := range m.messages {
		s += fmt.Sprintf("%s:\n%s\n\n", msg.role, msg.content)
	}

	s += fmt.Sprintf("%s\n\n", m.input)

	return s
}

func main() {
	_, err := tea.NewProgram(model{}).Run()

	if err != nil {
		fmt.Printf("Error: %v\n", err)
		os.Exit(1)
	}
}
