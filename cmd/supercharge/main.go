package main

import (
	"io"
	"log"
	"os"
	"net/http"
	"github.com/rivo/tview"
)

func main() {
	newPrimitive := func(text string) tview.Primitive {
		return tview.NewTextView().
			SetTextAlign(tview.AlignCenter).
			SetText(text)
	}
	menu := newPrimitive(fetchData("https://feed.infoq.com/news/"))
	main := newPrimitive("Main content")

	grid := tview.NewGrid().
		SetRows(3, 0, 3).
		SetColumns(60, 0).
		SetBorders(true).
		AddItem(newPrimitive("Header"), 0, 0, 1, 2, 0, 0, false).
		AddItem(newPrimitive("Footer"), 2, 0, 1, 2, 0, 0, false)

	grid.AddItem(menu, 1, 0, 1, 1, 0, 30, false).
		AddItem(main, 1, 1, 1, 1, 0, 30, false)

	if err := tview.NewApplication().SetRoot(grid, true).SetFocus(grid).Run(); err != nil {
		panic(err)
	}
}

func fetchData(url string) string {
	resp, err := http.Get(url)
	if err != nil {
		return ""
	}
	defer resp.Body.Close()

	body, err  := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatal("Failed to parse response body:", err)
		os.Exit(1)
	}

	return string(body)
}