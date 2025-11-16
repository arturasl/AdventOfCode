(ns d16-1.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]))

(defn str->num-list [s]
  (map #(Character/digit %1 10) s))

(def pattern [0 1 0 -1])

(defn expand-pattern [n]
  (->> pattern
       (map (partial repeat n))
       repeat
       flatten
       (drop 1)))

(deftest test-expand-pattern
  (is (= [1 0 -1 0 1]
         (take 5 (expand-pattern 1))))
  (is (= [0 0 1 1 1 0 0 0 -1 -1 -1]
         (take 11 (expand-pattern 3)))))

(defn next-phase [s]
  (let [cnt (count s)]
    (for [n (map inc (range cnt))]
      (mod
       (abs
        (reduce +
                (map #(* %1 %2)
                     s
                     (take cnt (expand-pattern n)))))
       10))))

(deftest test-next-phase
  (is (= [4 8 2 2 6 1 5 8]
         (next-phase [1 2 3 4 5 6 7 8]))))

(defn next-phase-rep [s n]
  ((apply comp (repeat n next-phase)) s))

(deftest test-next-phase-rep
  (is (= [0 1 0 2 9 4 9 8]
         (next-phase-rep [1 2 3 4 5 6 7 8] 4))))

(defn first-eight [s]
  (apply str (take 8 (next-phase-rep (str->num-list s) 100))))

(deftest test-first-eight
  (is (= "24176176"
         (first-eight "80871224585914546619083218645595")))
  (is (= "73745418"
         (first-eight "19617804207202209144916044189917")))
  (is (= "52432133"
         (first-eight "69317163492948606335995924319873"))))

(defn -main [& _]
  (println (first-eight (str/trim (slurp "./large.in")))))
