package ui

import (
	chat "supercharge/internal/chat"
	menu "supercharge/internal/menu"
	notes "supercharge/internal/notes"

	tea "github.com/charmbracelet/bubbletea"
)

type scene int

const (
	sceneMenu scene = iota
	sceneChat
	sceneNotes
)

type Root struct {
	active scene
	menu   menu.Model
	chat   chat.Model
	notes  notes.Model
}

func New() *Root {
	return &Root{
		active: sceneMenu,
		menu:   menu.New(),
		chat:   chat.New(),
		notes:  notes.New(),
	}
}

func (m *Root) Init() tea.Cmd {
	return m.menu.Init()
}

func (m *Root) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var cmd tea.Cmd

	switch msg := msg.(type) {

	case menu.SelectMsg:

		switch msg.Item {
		case "chat":
			m.active = sceneChat
			cmd = m.chat.Init()

		case "notes":
			m.active = sceneNotes
			cmd = m.notes.Init()
		}
	}

	switch m.active {
	case sceneMenu:
		m.menu, cmd = m.menu.Update(msg)
	case sceneChat:
		m.chat, cmd = m.chat.Update(msg)
	case sceneNotes:
		m.notes, cmd = m.notes.Update(msg)
	}

	return m, cmd
}

func (m *Root) View() string {
	switch m.active {
	case sceneMenu:
		return m.menu.View()
	case sceneChat:
		return m.chat.View()
	case sceneNotes:
		return m.notes.View()
	default:
		return ""
	}
}
