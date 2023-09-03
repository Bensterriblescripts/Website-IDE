package main

import (
	"fmt"
	"log"
	"net/http"
)

func main() {
	// HTTP Server
	fileServer := http.FileServer(http.Dir("./html"))
	http.Handle("/", fileServer)
	fmt.Printf("Starting the HTTP server on port 8000\n")
	if err := http.ListenAndServe(":8000", nil); err != nil {
		log.Fatal(err)
	}
}
