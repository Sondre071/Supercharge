package menu

import (
	"fmt"
	tea "github.com/charmbracelet/bubbletea"
	"os"
)

const reset = "\033[0m"

// const blue = "\033[34m"
// const cyan = "\033[36m"
// const gray = "\033[37m"
// const green = "\033[32m"
// const magenta = "\033[35m"
// const red = "\033[31m"
// const white = "\033[97m"
const yellow = "\033[33m"

type Menu struct {
	title   string
	choices []string
	next    []*Menu
}

var mainMenu = &Menu{}
var openrouterMenu = &Menu{}
var settingsMenu = &Menu{}

func init() {
	*mainMenu = Menu{
		title:   "Main menu",
		choices: []string{"OpenRouter", "Settings", "Exit"},
		next:    []*Menu{openrouterMenu, settingsMenu, nil},
	}

	*openrouterMenu = Menu{
		title:   "OpenRouter",
		choices: []string{"New session", "Prompts", "Settings", "Back"},
		next:    []*Menu{nil, nil, nil, mainMenu},
	}

	*settingsMenu = Menu{
		title:   "Settings",
		choices: []string{"Models", "Prompts", "Back"},
		next:    []*Menu{nil, nil, mainMenu},
	}
}

type Model struct {
	menu   *Menu
	cursor int
}

func (m Model) Init() tea.Cmd { return nil }

func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {

	case tea.KeyMsg:
		switch msg.String() {

		case "ctrl+c", "q", "h":
			return m, tea.Quit

		case "up", "k":
			if m.cursor > 0 {
				m.cursor--
			}

		case "down", "j":
			if m.cursor < len(m.menu.choices)-1 {
				m.cursor++
			}

		case "enter", "l":

			nextMenu := m.menu.next[m.cursor]
			if nextMenu == nil {
				return m, tea.Quit
			}

			return Model{menu: nextMenu, cursor: 0}, nil
		}
	}

	return m, nil
}

func (m Model) View() string {
	s := yellow + m.menu.title + "\n" + reset

	for i, choice := range m.menu.choices {
		cursor := " "

		if m.cursor == i {
			cursor = ">"
		}

		s += fmt.Sprintf("%s %s\n", cursor, choice)
	}

	return s
}

func Run() {
	print("\n")

	p := tea.NewProgram(Model{
		menu:   mainMenu,
		cursor: 0,
	})

	if _, err := p.Run(); err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}
}
