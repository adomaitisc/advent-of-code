package main

import (
	"fmt"
	"os"
	"strconv"
	"sync"
	"time"
)

type Almanac struct {
	Name string
	Values [][]int
	Maps []Map
}

type Map struct {
	Source Source
	Destination Source
	Length int
	Difference int
}

type Source struct {
	Start int
	End int
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func LoadAlmanac(data []byte) []Almanac {
	data_as_string := string(data[:])

	mappings := make([]Almanac, 0)

	var mapping Almanac
	var numbers []int
	var number string
	var name string
	var had_newline bool

	for _, char := range data_as_string {
		if char != '\n' {
			had_newline = false
		}

		// if char is a number...
		if (char >= '0' && char <= '9') {
			number += string(char)
			continue
		}

		// if char is a character...
		if char >= 'a' && char <= 'z' || char == '-'  {
			name += string(char)
			continue
		}

		// if char is a colon...
		if char == ':' && len(mappings) > 0{
			// clear the name
			name = ""
			continue
		}

		// if char is a space...
		if char == ' ' {
			// check if theres a name or a number
			if name != "" {
				mapping.Name = name
				name = ""
			}

			if number != "" {
				// convert number to int
				number_as_int, err := strconv.Atoi(number)
				check(err)

				numbers = append(numbers, number_as_int)
				number = ""
			}
			continue
		}

		if char == '\n' {
			if had_newline && mapping.Name != "" && len(mapping.Values) > 0 {
				for _, value := range mapping.Values {

					s := Source{Start: value[1], End: value[1] + value[2] - 1}
					d := Source{Start: value[0], End: value[0] + value[2] - 1}

					m := Map{Source: s, Destination: d, Length: value[2], Difference: value[0] - value[1]}
					mapping.Maps = append(mapping.Maps, m)
				}

				mappings = append(mappings, mapping)

				mapping = Almanac{}
				had_newline = false
				continue
			}
			// check if theres a number and add it to the mapping
			if number != "" {
				// convert number to int
				number_as_int, err := strconv.Atoi(number)
				check(err)

				numbers = append(numbers, number_as_int)
				mapping.Values = append(mapping.Values, numbers)

				numbers = make([]int, 0)
				number = ""
			}
			had_newline = true
		}
	}
	return mappings
}

func ApplyDifference(seed int, maps []Map) int {
	for i := range maps {
		m := maps[i]

		// if seed is in the range of the source
		if seed >= m.Source.Start && seed < m.Source.End {
			// apply the difference to the seed
			seed += m.Difference
			break
		}
	}
	return seed
}

func LocationWorker(seed_start int, seed_range int, mappings []Almanac, results chan int, wg *sync.WaitGroup) {
	defer wg.Done()
	smallest := seed_start
	
	for i := seed_start; i < seed_start + seed_range; i++ {
		seed := i

		// apply the difference to the seed
		for j := 1 ; j < len(mappings) ; j++ {
			seed = ApplyDifference(seed, mappings[j].Maps)
		}

		if seed < smallest {
			smallest = seed
		}
	}

	results <- smallest
}


func main() {	

	// get current time
	t := time.Now()
	
	data, err := os.ReadFile("input")
	check(err)
	
	mappings := LoadAlmanac(data)
	
	seed_range := mappings[0].Values[0]
	fmt.Println("Seed Range: ", seed_range)

	// Create a channel to collect the results
	results := make(chan int, len(seed_range)/2)

	var wg sync.WaitGroup // 1
	// Create a goroutine for each index
	for i := range seed_range {
		if i % 2 == 0 {
			wg.Add(1)
			go LocationWorker(seed_range[i], seed_range[i + 1], mappings, results, &wg)
		}
	}

	smallest := seed_range[0]

	// Collect and print the results from all goroutines
	for i := 0; i <  len(seed_range)/2; i++ {
		if result := <-results; result < smallest {
			smallest = result
		}
	}
		
	// Wait for all goroutines to finish
	wg.Wait()
	close(results)

	// print the time taken
	fmt.Println("Time taken: ", time.Since(t))

	fmt.Println("Smallest: ", smallest)
}