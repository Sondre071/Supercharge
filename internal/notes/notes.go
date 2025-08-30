package notes

import (
	tea "github.com/charmbracelet/bubbletea"
)

type note struct {
	title string
	desc  string
}

type Model struct {
	notes []note
}

func New() Model {
	return Model{notes: []note{{title: "First note!", desc: "yes:)"}}}
}

func (m Model) Init() tea.Cmd {
	return nil
}

func (m Model) Update(msg tea.Msg) (Model, tea.Cmd) {
	return m, nil
}

func (m Model) View() string {
	return "notes!"
}
