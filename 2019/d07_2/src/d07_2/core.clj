(ns d07-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [clojure.set :as set]
            [clojure.math.combinatorics :as combo]
            [clojure.data.finger-tree :as finger]
            [intcode.core :as code]))

(defn map-subset? [subset superset]
  (set/subset? (set subset) (set superset)))

(defn run-in-seq [program phase-settings]
  (loop [programs (update (vec (map #(code/to->stdin program [%])
                                    phase-settings))
                          0
                          #(code/to->stdin % [0]))
         vals []]
    (let [next-state
          (reduce
           (fn [{:keys [next-programs next-vals]} program]
             (let [program (code/exec (code/to->stdin program next-vals))]
               (assert (or
                        (= (count (:output program)) 1)
                        (= (:state program) :halt))
                       (str (:output program) (:state program)))
               {:next-programs (conj next-programs
                                     (assoc program :output (finger/double-list)))
                :next-vals (vec (:output program))}))
           {:next-programs [] :next-vals vals}
           programs)]
      (if (every? #(= (:state %) :halt) (:next-programs next-state))
        (last (:next-vals next-state))
        (recur (:next-programs next-state)
               (:next-vals next-state))))))

(deftest test-run-in-seq
  (is (= 139629729
         (run-in-seq (code/init-program
                      [3 26 1001 26 -4 26 3 27 1002 27 2 27 1 27 26 27 4 27
                       1001 28 -1 28 1005 28 6 99 0 0 5])
                     [9 8 7 6 5])))
  (is (= 18216
         (run-in-seq (code/init-program
                      [3 52 1001 52 -5 52 3 53 1 52 56 54 1007 54 5 55 1005 55
                       26 1001 54 -5 54 1105 1 12 1 53 54 53 1008 54 0 55 1001
                       55 1 55 2 53 55 53 4 53 1001 56 -1 56 1005 56 6 99 0 0 0
                       0 10])
                     [9 7 8 5 6]))))

(defn solve [s]
  (let [program (code/init-program (code/str->memory s))]
    (reduce max (map (partial run-in-seq program) (combo/permutations (range 5 (+ 9 1)))))))

(deftest test-solve
  (is (= 139629729
         (solve "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"))))

(defn -main
  [& _]
  (println (solve (slurp  *in*))))
