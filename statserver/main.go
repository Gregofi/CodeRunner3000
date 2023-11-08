package main

import (
    "fmt"
    "net/http"
    "os"
    "io"
)

var (
    port = os.Getenv("PORT")
    endpoints = []string{
        "evaluator:7800/metrics",
    }
)

func allMetrics(w http.ResponseWriter, r *http.Request) {
    metrics := ""
    for _, endpoint := range endpoints {
        resp, err := http.Get("http://" + endpoint)
        if err != nil {
            fmt.Fprintf(w, "Error: %s\n", err)
            return
        }
        defer resp.Body.Close()

        b, err := io.ReadAll(resp.Body)
        if err != nil {
            fmt.Fprintf(w, "Error fetching metrics from endpoint %s: %s\n", endpoint, err)
            continue
        }

        metrics += "# Source: " + endpoint + "\n"
        metrics += string(b) + "\n";
    }
    w.Write([]byte(metrics))
}

func main() {
    if port == "" {
        port = "9999"
    }
    fmt.Printf("Listening on port %s\n", port)
    http.HandleFunc("/metrics", allMetrics)
    err := http.ListenAndServe(":" + port, nil)
    if err != nil {
        panic(err)
    }
}
