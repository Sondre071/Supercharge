package menu

import (
	tea "github.com/charmbracelet/bubbletea"
)

type Model struct {
	cursor int
	items  []string
}

type SelectMsg struct {
	Item string
}

func New() Model {
	return Model{items: []string{"Chat", "Notes"}, cursor: 0}
}

func (m Model) Init() tea.Cmd {
	return nil
}

func (m Model) Update(msg tea.Msg) (Model, tea.Cmd) {
	var cmd tea.Cmd

	switch msg := msg.(type) {

	case tea.KeyMsg:

		switch msg.String() {

		case "up", "k":
			if m.cursor > 0 {
				m.cursor--
			}

		case "down", "j":
			if m.cursor < len(m.items)-1 {
				m.cursor++
			}

		case "ctrl+c", "q":
			cmd = tea.Quit

		case "enter", "l":
			cmd = func() tea.Msg {
				return SelectMsg{Item: m.items[m.cursor]}
			}
		}
	}

	return m, cmd
}

func (m Model) View() string {
	s := "Main menu:\n\n"

	for i, item := range m.items {
		cursor := " "
		if m.cursor == i {
			cursor = ">"
		}
		s += cursor + " " + item + "\n"
	}

	return s
}
