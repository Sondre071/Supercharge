package main

import (
	"fmt"

	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

var docStyle = lipgloss.NewStyle().Margin(1, 2)

type item struct {
	title, desc string
}

func (i item) Title() string       { return i.title }
func (i item) Description() string { return i.desc }
func (i item) FilterValue() string { return i.title }

type listModel struct {
	list     list.Model
	selected *item
}

func (m listModel) Init() tea.Cmd {
	return nil
}

func (m listModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {

	case tea.KeyMsg:

		switch msg.String() {

		case "enter", "l":
			if it, ok := m.list.SelectedItem().(item); ok {
				m.selected = &it
				return m, tea.Quit
			}

		case "q":
			return m, tea.Quit
		}

	case tea.WindowSizeMsg:
		h, v := docStyle.GetFrameSize()
		m.list.SetSize(msg.Width-h, msg.Height-v)
	}

	var cmd tea.Cmd
	m.list, cmd = m.list.Update(msg)
	return m, cmd
}

func (m listModel) View() string {
	return docStyle.Render(m.list.View())
}

func runList() (*item, error) {
	items := []list.Item{
		item{title: "My first note", desc: "Wow..."},
		item{title: "My SECOND NOTE!", desc: "Wow..."},
		item{title: "My THIRDDD NOTE!", desc: "Wow..."},
	}

	m := listModel{list: list.New(items, list.NewDefaultDelegate(), 0, 0)}
	m.list.Title = "My notes!"

	p := tea.NewProgram(m, tea.WithAltScreen())

	finalModel, err := p.Run()
	if err != nil {
		return nil, fmt.Errorf("run list: %w", err)
	}

	lm, ok := finalModel.(listModel)
	if !ok {
		return nil, fmt.Errorf("unexpected model type %T", finalModel)
	}

	return lm.selected, nil
}
