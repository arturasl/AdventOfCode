(ns d11-1.core-test
  (:require [d11-1.core :refer :all]
            [clojure.test :refer [deftest is]]
            [clojure.set :as set]))

(defn- map-subset? [subset superset]
  (set/subset? (set subset) (set superset)))

(deftest test-str->memory
  (is (= [2 3 0 3 99] (str->memory "2,3,0,3, 99"))))

(deftest test-to->stdin
  (is (= {:input [1 2 3] :state :ready}
         (to->stdin {:input [1 2] :state :waiting-read} [3])))
  (is (= {:input [1 2] :state :waiting-read}
         (to->stdin {:input [1 2] :state :waiting-read} nil)))
  (is (= {:input [1 2] :state :waiting-read}
         (to->stdin {:input [1 2] :state :waiting-read} [])))
  (is (= {:input [1 2 3] :state :halt}
         (to->stdin {:input [1 2] :state :halt} [3]))))

(deftest test-parse-instruction
  (is (= {:op :mul :addressings [:position :immediate :position] :pointer 0 :num-params 3}
         (parse-instruction {:memory {0 1002} :pointer 0})))
  (is (= {:op :mul :addressings [:position :position :position] :pointer 0 :num-params 3}
         (parse-instruction {:memory {0 2} :pointer 0}))))

(deftest test-exec
  (let [exec-get-output (fn [memory input]
                          (-> (init-program {:memory memory :input input})
                              exec
                              (:output)))
        exec-single-output (comp last exec-get-output)]
    ; Day 2  part 1.
    (is (map-subset? {:memory (memory->map [2 0 0 0 99]) :pointer 4}
                     (exec (init-program {:memory [1 0 0 0 99] :pointer 0}))))
    (is (map-subset? {:memory (memory->map [2 3 0 6 99]) :pointer 4}
                     (exec (init-program {:memory [2 3 0 3 99] :pointer 0}))))
    (is (map-subset? {:memory (memory->map [2 4 4 5 99 9801]) :pointer 4}
                     (exec (init-program {:memory [2 4 4 5 99 0] :pointer 0}))))
    (is (map-subset? {:memory (memory->map [30 1 1 4 2 5 6 0 99]) :pointer 8}
                     (exec (init-program {:memory [1 1 1 4 99 5 6 0 99] :pointer 0}))))
    (is (map-subset? {:memory (memory->map [3500 9 10 70 2 3 11 0 99 30 40 50]) :pointer 8}
                     (exec (init-program {:memory [1 9 10 3 2 3 11 0 99 30 40 50] :pointer 0}))))

    ; Day 5  part 1.
    (is (map-subset? {:memory (memory->map [1 2 3 6 99]) :pointer 4}
                     (exec (init-program {:memory [1 2 3 3 99] :pointer 0}))))
    (is (map-subset? {:memory (memory->map [42 0 4 0 99]) :input [] :output [42] :pointer 4}
                     (exec (init-program {:memory [3 0 4 0 99] :input [42] :output [] :pointer 0}))))

    ; Day 5  part 2.
    ; Is input 8? (position mode)
    (let [memory [3 9 8 9 10 9 4 9 99 -1 8]]
      (is (= 1 (exec-single-output memory [8])))
      (is (= 0 (exec-single-output memory [7]))))
    ; Is input less than 8? (position mode)
    (let [memory [3 9 7 9 10 9 4 9 99 -1 8]]
      (is (= 1 (exec-single-output memory [7])))
      (is (= 0 (exec-single-output memory [9]))))
    ; Is input 8? (immediate mode)
    (let [memory [3 3 1108 -1 8 3 4 3 99]]
      (is (= 1 (exec-single-output memory [8])))
      (is (= 0 (exec-single-output memory [7]))))
    ; Is input less than 8? (immediate mode)
    (let [memory [3 3 1107 -1 8 3 4 3 99]]
      (is (= 1 (exec-single-output memory [7])))
      (is (= 0 (exec-single-output memory [9]))))
    ; (bool)input (position)
    (let [memory [3 12 6 12 15 1 13 14 13 4 13 99 -1 0 1 9]]
      (is (= 0 (exec-single-output memory [0])))
      (is (= 1 (exec-single-output memory [1])))
      (is (= 1 (exec-single-output memory [2]))))
    ; (bool)input (immediate)
    (let [memory [3 3 1105 -1 9 1101 0 0 12 4 12 99 1]]
      (is (= 0 (exec-single-output memory [0])))
      (is (= 1 (exec-single-output memory [1])))
      (is (= 1 (exec-single-output memory [2]))))
    ; input < 8 => 999; input = 8 => 1000; input > 8 => 1001
    (let [memory [3 21 1008 21 8 20 1005 20 22 107 8 21 20 1006 20 31 1106 0 36
                  98 0 0 1002 21 125 20 4 20 1105 1 46 104 999 1105 1 46 1101
                  1000 1 20 4 20 1105 1 46 98 99]]
      (is (= 999 (exec-single-output memory [7])))
      (is (= 1000 (exec-single-output memory [8])))
      (is (= 1001 (exec-single-output memory [9]))))

    ; Day 9  part 1.
    (is (= [42] (:output (exec (init-program {:memory {0 109 1 19 2 204 3 -34 4 99 1985 42} :relative-base 2000})))))
    (let [memory [109 1 204 -1 1001 100 1 100 1008 100 16 101 1006 101 0 99]]
      (is (= memory (exec-get-output memory []))))
    (is (= 16 (count (str (exec-single-output [1102 34915192 34915192 7 4 7 99 0] [])))))
    (is (= 1125899906842624 (exec-single-output [104 1125899906842624 99] [])))))
