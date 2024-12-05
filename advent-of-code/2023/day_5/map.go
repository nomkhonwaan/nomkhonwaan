package main

import (
	"strconv"
	"strings"
)

type Map struct {
	records Records
}

func (m Map) GetDestination(target uint64) uint64 {
	for _, r := range m.records {
		if r.IsSourceInRange(target) {
			return r.GetDestination(target)
		}
	}
	return target
}

func (m Map) GetSource(target uint64) uint64 {
	for _, r := range m.records {
		if r.IsDestinationInRange(target) {
			return r.GetSource(target)
		}
	}
	return target
}

type Record struct {
	src, dst uint64
	length   uint64
}

type Records []Record

func (r Records) Len() int           { return len(r) }
func (r Records) Less(i, j int) bool { return r[i].dst > r[j].dst } // sort by descending
func (r Records) Swap(i, j int)      { r[i], r[j] = r[j], r[i] }

func (r Record) IsSourceInRange(target uint64) bool {
	return r.src <= target && target <= r.src+r.length-1
}

func (r Record) IsDestinationInRange(target uint64) bool {
	return r.dst <= target && target <= r.dst+r.length-1
}

func (r Record) GetSource(target uint64) uint64 {
	return r.src + (target - r.dst)
}

func (r Record) GetDestination(target uint64) uint64 {
	return r.dst + (target - r.src)
}

func parseMap(lines []string) Map {
	m := Map{records: make([]Record, 0)}
	for _, line := range lines {
		v := strings.Split(line, " ")
		src, _ := strconv.ParseUint(v[1], 10, 64)
		dst, _ := strconv.ParseUint(v[0], 10, 64)
		length, _ := strconv.ParseUint(v[2], 10, 64)
		m.records = append(m.records, Record{src: src, dst: dst, length: length})
	}
	return m
}
