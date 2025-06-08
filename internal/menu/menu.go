package menu

import (
	"fmt"
	tea "github.com/charmbracelet/bubbletea"
	"os"
)

const Reset = "\033[0m"

// const blue = "\033[34m"
const Cyan = "\033[36m"
// const gray = "\033[37m"
// const green = "\033[32m"
// const magenta = "\033[35m"
// const red = "\033[31m"
// const white = "\033[97m"
const Yellow = "\033[33m"

type Choice struct {
	Title string
	Next func() *Menu
}

type Menu struct {
	Title   string
	Choices []Choice
}

type Model struct {
	Menu   *Menu
	Cursor int
}

func (m Model) Init() tea.Cmd { return nil }

func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {

	case tea.KeyMsg:
		switch msg.String() {

		case "ctrl+c", "q", "h":
			return m, tea.Quit

		case "up", "k":
			if m.Cursor > 0 {
				m.Cursor--
			}

		case "down", "j":
			if m.Cursor < len(m.Menu.Choices)-1 {
				m.Cursor++
			}

		case "enter", "l":

			nextFunc := m.Menu.Choices[m.Cursor].Next
			if nextFunc == nil {
				return m, tea.Quit
			}

			nextMenu := nextFunc()

			if nextMenu == nil {
				return m, tea.Quit
			}

			return Model{Menu: nextMenu, Cursor: 0}, nil
		}
	}

	return m, nil
}

func (m Model) View() string {
	s := Yellow + m.Menu.Title + "\n" + Reset

	for i, choice := range m.Menu.Choices {
		Cursor := " "

		if m.Cursor == i {
			Cursor = ">"
		}

		s += fmt.Sprintf("%s %s\n", Cursor, choice.Title)
	}

	return s
}

func Run(model Model) {
	print("\n")

	p := tea.NewProgram(model)

	if _, err := p.Run(); err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}
}
