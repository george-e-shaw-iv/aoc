(ns aoc.2022.day-one
  "AOC Day One Part One"
  (:require 
    [clojure.string :as str]))
(def inventories 
  (for [inventory (str/split (slurp "input.txt") #"\n\n")] (map read-string (str/split inventory #"\n"))))
(def calories
  (for [inventory inventories] (reduce + inventory)))
(def maxCalories
  (apply max calories))
(println maxCalories)
