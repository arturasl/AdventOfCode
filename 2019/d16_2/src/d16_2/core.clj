(ns d16-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]))

(defn str->num-vec [s]
  (vec (map #(Character/digit %1 10) s)))

(defn get-ranges [s cnt]
  (let [s_len (count s)
        prefix_sum (reduce (fn [acc el] (conj acc (+ (last acc) el))) [0] s)]
    (for [row (range s_len)]
      (let [jump (* (inc row) 4)
            len (inc row)
            start (if (= cnt 1) row (+ 2 (* row 3)))]
        (loop [i start result 0]
          (if (>= i s_len) result
              (recur
               (+ i jump)
               (+ result (- (nth prefix_sum (min (+ i len) s_len)) (nth prefix_sum i))))))))))

(defn next-phase [s]
  (vec (map
        #(mod (abs (- %1 %2)) 10)
        (get-ranges s 1)
        (get-ranges s -1))))

(deftest test-next-phase
  (is (= [4 8 2 2 6 1 5 8]
         (next-phase [1 2 3 4 5 6 7 8]))))

(defn next-phase-rep [s n]
  ((apply comp (repeat n next-phase)) s))

(deftest test-next-phase-rep
  (is (= [0 1 0 2 9 4 9 8]
         (next-phase-rep [1 2 3 4 5 6 7 8] 4))))

(defn first-eight [s]
  (apply str (take 8 (next-phase-rep (flatten (repeat 5 (str->num-vec s))) 100))))

(deftest test-first-eight
  (is (= "24176176"
         (first-eight "80871224585914546619083218645595")))
  (is (= "73745418"
         (first-eight "19617804207202209144916044189917")))
  (is (= "52432133"
         (first-eight "69317163492948606335995924319873"))))

(defn -main [& _]
  (println (first-eight (str/trim (slurp "./large.in")))))
