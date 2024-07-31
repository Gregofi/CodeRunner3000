package main

import (
	"fmt"
	"regexp"
	"strconv"

	"github.com/go-redis/redis"
)

type RedisCollector struct {
	appName string
	redis   *redis.Client
}

func NewRedisCollector(appName string, host string, port int) RedisCollector {
	Addr := fmt.Sprintf("%s:%d", host, port)
	return RedisCollector{
		appName: appName,
		redis: redis.NewClient(&redis.Options{
			Addr:     Addr,
			Password: "",
			DB:       0,
		}),
	}
}

func parseSimpleMetric(name string, info string) (float64, error) {
	re := regexp.MustCompile(fmt.Sprintf("%s:(\\d+)", name))
	match := re.FindStringSubmatch(info)
	if len(match) == 0 {
		return 0, fmt.Errorf("metric %s not found", name)
	}
	value, err := strconv.ParseFloat(match[1], 64)
	if err != nil {
		return 0, fmt.Errorf("error parsing metric %s: %s", name, err)
	}
	return value, nil
}

func (r RedisCollector) extractRedisMetrics() (*string, error) {
	result := fmt.Sprintf("# Source: redis://%s\n", r.redis.Options().Addr)
	infoString := r.redis.Info().String()

	metrics := []string{
		"used_memory",
		"used_memory_peak",
	}

	for _, metric := range metrics {
		value, err := parseSimpleMetric(metric, infoString)
		if err != nil {
			return nil, err
		}
		result += fmt.Sprintf("redis_%s{type=\"%s\"} %f\n", metric, r.appName, value)
	}

	re := regexp.MustCompile("db0:keys=(\\d+)")
	match := re.FindStringSubmatch(infoString)
	if len(match) == 0 {
		return nil, fmt.Errorf("metric keys not found")
	}
	keys, err := strconv.Atoi(match[1])
	if err != nil {
		return nil, fmt.Errorf("error parsing metric keys: %s", err)
	}
	result += fmt.Sprintf("redis_keys{type=\"%s\"} %d\n", r.appName, keys)

	return &result, nil
}
