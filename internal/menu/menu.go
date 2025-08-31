package menu

import (
	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"strings"
)

type item struct {
	title string
	desc  string
}

func (i item) Title() string       { return i.title }
func (i item) Description() string { return i.desc }
func (i item) FilterValue() string { return i.title }

type Model struct {
	list     list.Model
	docStyle lipgloss.Style
}

type SelectMsg struct {
	Item  string
	Index int
}

func New() Model {
	items := []list.Item{
		item{title: "Chat", desc: "Chat with an LLM."},
		item{title: "Notes", desc: "Write and browse notes."},
	}

	l := list.New(items, list.NewDefaultDelegate(), 0, 0)
	l.Title = "Main menu"
	l.SetShowStatusBar(false)
	l.SetFilteringEnabled(false)
	l.SetShowHelp(true)
	l.SetShowPagination(true)

	return Model{
		list:     l,
		docStyle: lipgloss.NewStyle().Margin(1, 2),
	}
}

func (m Model) Init() tea.Cmd {
	return nil
}

func (m Model) Update(msg tea.Msg) (Model, tea.Cmd) {
	var cmd tea.Cmd

	switch msg := msg.(type) {

	case tea.WindowSizeMsg:
		h, v := m.docStyle.GetFrameSize()
		m.list.SetSize(msg.Width-h, msg.Height-v)

	case tea.KeyMsg:
		if msg.String() == "enter" {
			if it, ok := m.list.SelectedItem().(item); ok {
				idx := m.list.Index()
				return m, func() tea.Msg {
					return SelectMsg{
						Item: strings.ToLower(it.title),
						Index: idx,
					}
				}
			}
		}



	}

	m.list, cmd = m.list.Update(msg)

	return m, cmd
}

func (m Model) View() string {
	return m.docStyle.Render(m.list.View())
}
