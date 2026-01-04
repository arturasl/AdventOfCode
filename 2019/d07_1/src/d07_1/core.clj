(ns d07-1.core
  (:gen-class)
  (:require [clojure.test :refer [deftest is]]
            [clojure.math.combinatorics :as combo]
            [intcode.core :as code]))

(defn run-in-seq [mem phase-settings]
  (let [program (code/init-program mem)]
    (reduce (fn [input phase-setting]
              (as-> [phase-setting input] v
                (code/to->stdin program v)
                (code/exec v)
                (get v :output)
                (last v)))
            0
            phase-settings)))

(deftest test-run-in-seq
  (is (= 43210 (run-in-seq [3 15 3 16 1002 16 10 16 1 16 15 15 4 15 99 0 0] [4 3 2 1 0])))
  (is (= 54321 (run-in-seq [3 23 3 24 1002 24 10 24 1002 23 -1 23 101 5 23 23 1 24 23 23 4 23 99 0 0] [0 1 2 3 4])))
  (is (= 65210 (run-in-seq [3 31 3 32 1002 32 10 32 1001 31 -2 31 1007 31 0 33 1002 33 7 33 1 33 31 31 1 32 31 31 4 31 99 0 0 0] [1 0 4 3 2]))))

(defn solve [s]
  (let [mem (code/str->memory s)]
    (reduce max (map (partial run-in-seq mem)
                     (combo/permutations (range 5))))))

(deftest test-solve
  (is (= 43210 (solve "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"))))

(defn -main
  [& _]
  (println (solve (slurp  *in*))))
