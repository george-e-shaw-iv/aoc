(ns aoc.2022.day-one.part-two
  "AOC Day One Part Two"
  (:require 
    [clojure.string :as str]))
(def inventories 
  (for [inventory (str/split (slurp "input.txt") #"\n\n")] (map read-string (str/split inventory #"\n"))))
(def calories
  (reverse (sort (for [inventory inventories] (reduce + inventory)))))
(def topThreeCalories
  (+ (nth calories 0) (nth calories 1) (nth calories 2)))
(println topThreeCalories)
