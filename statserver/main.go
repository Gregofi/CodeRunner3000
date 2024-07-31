package main

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"strconv"
)

type nginxEndpoint struct {
	endpoint      string
	authorization string
}

var (
	port      = os.Getenv("PORT")
	endpoints = []string{
		"evaluator:7800/metrics",
	}
	nginx_endpoints = []nginxEndpoint{
		{
			endpoint:      "website-proxy:80/nginx_status",
			authorization: os.Getenv("WEBSITE_PROXY_NGINX_STATUS_TOKEN"),
		},
	}
	redisEndpoints = []RedisCollector{
		NewRedisCollector("links", os.Getenv("REDIS_LINKS_HOST"), ignoreError(strconv.Atoi(os.Getenv("REDIS_LINKS_PORT")))),
	}
)

func ignoreError[T any](first T, _ any) T {
	return first
}

func allMetrics(w http.ResponseWriter, r *http.Request) {
	metrics := ""
	for _, endpoint := range endpoints {
		resp, err := http.Get("http://" + endpoint)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %s\n", err)
			continue
		}
		defer resp.Body.Close()

		b, err := io.ReadAll(resp.Body)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error fetching metrics from endpoint %s: %s\n", endpoint, err)
			continue
		}

		metrics += "# Source: " + endpoint + "\n"
		metrics += string(b) + "\n"
	}
	for _, endpoint := range nginx_endpoints {
		req, err := http.NewRequest("GET", "http://"+endpoint.endpoint, nil)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %s\n", err)
			continue
		}
		req.Header.Set("Authorization", "Bearer "+endpoint.authorization)

		resp, err := http.DefaultClient.Do(req)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %s\n", err)
			continue
		}
		defer resp.Body.Close()

		b, err := io.ReadAll(resp.Body)
		if err != nil {
			fmt.Fprintf(w, "Error fetching metrics from endpoint %s: %s\n", endpoint, err)
			continue
		}

		parsed := parseNginxMetrics(string(b))
		metrics += "# Source: " + endpoint.endpoint + "\n"
		metrics += parsed + "\n"
	}
	for _, redisEndpoint := range redisEndpoints {
		redisMetrics, err := redisEndpoint.extractRedisMetrics()
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error fetching metrics from redis: %s\n", err)
			continue
		}
		metrics += *redisMetrics
	}

	w.Write([]byte(metrics))
}

func parseNginxMetrics(metrics string) string {
	var active, accepts, handled, requests, reading, writing, waiting int
	fmt.Sscanf(metrics, "Active connections: %d\nserver accepts handled requests\n %d %d %d\nReading: %d Writing: %d Waiting: %d\n", &active, &accepts, &handled, &requests, &reading, &writing, &waiting)
	parsedMetrics := fmt.Sprintf("nginx_active_connections %d\nnginx_accepts %d\nnginx_handled %d\nnginx_requests %d\nnginx_reading %d\nnginx_writing %d\nnginx_waiting %d\n", active, accepts, handled, requests, reading, writing, waiting)
	return parsedMetrics
}

func main() {
	if port == "" {
		port = "9999"
	}
	fmt.Printf("Listening on port %s\n", port)
	http.HandleFunc("/metrics", allMetrics)
	err := http.ListenAndServe(":"+port, nil)
	if err != nil {
		panic(err)
	}
}
