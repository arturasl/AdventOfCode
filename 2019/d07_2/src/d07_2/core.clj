(ns d07-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [clojure.set :as set]
            [clojure.math.combinatorics :as combo]))

(defn str->memory [s]
  (->> (str/split s #",")
       (map str/trim)
       (remove empty?)
       (map parse-long)
       (into [])))

(deftest test-str->memory
  (is (= [2 3 0 3 99] (str->memory "2,3,0,3, 99"))))

(defn init-program [base]
  (if (map? base)
    (merge {:memory [99] :input [] :output [] :pointer 0 :state :ready} base)
    (init-program {:memory base})))

(defn to->stdin [program vals]
  (let [program (update program :input into vals)]
    (if (and (not-empty vals) (= (:state program) :waiting-read))
      (assoc program :state :ready)
      program)))

(deftest test-to->stdin
  (is (= {:input [1 2 3] :state :ready}
         (to->stdin {:input [1 2] :state :waiting-read} [3])))
  (is (= {:input [1 2] :state :waiting-read}
         (to->stdin {:input [1 2] :state :waiting-read} nil)))
  (is (= {:input [1 2] :state :waiting-read}
         (to->stdin {:input [1 2] :state :waiting-read} [])))
  (is (= {:input [1 2 3] :state :halt}
         (to->stdin {:input [1 2] :state :halt} [3]))))

(defn opcode->op [opcode]
  (case (mod opcode 100)
    1 :sum
    2 :mul
    3 :read
    4 :write
    5 :jump-if-true
    6 :jump-if-false
    7 :less-than
    8 :equal
    99 :halt
    (throw (ex-info "Unknown opcode" {:opcode opcode}))))

(deftest test-opcode->op
  (is (= :read (opcode->op 3))))

(defn op->num-params [op]
  (case op
    :sum 3
    :mul 3
    :read 1
    :write 1
    :jump-if-true 2
    :jump-if-false 2
    :less-than 3
    :equal 3
    :halt 0
    (throw (ex-info "Unknown op" {:op op}))))

(defn parse-addressing [mode]
  (case mode
    \0 :position
    \1 :immediate
    (throw (ex-info "Unknown addressing:" {:mode mode}))))

(defn parse-instruction [memory pointer]
  (let [opcode (get memory pointer)
        op (opcode->op opcode)
        num-params (op->num-params op)
        str-addressings (reverse (if (>= opcode 100) (str (quot opcode 100)) ""))
        str-addressings-full (->> (repeat (- num-params (count str-addressings)) \0)
                                  (concat str-addressings)
                                  (apply str))
        addressings (vec (map parse-addressing str-addressings-full))]
    {:op op
     :addressings addressings
     :pointer pointer
     :num-params num-params}))

(deftest test-parse-instruction
  (is (= {:op :mul :addressings [:position :immediate :position] :pointer 0 :num-params 3}
         (parse-instruction [1002 1 1 1] 0)))
  (is (= {:op :mul :addressings [:position :position :position] :pointer 0 :num-params 3}
         (parse-instruction [2 1 1 1] 0))))

(defn resolve-raw-param [program instruction pos]
  (get (:memory program) (+ (:pointer instruction) pos 1)))

(defn resolve-param [program instruction pos]
  (let [memory (:memory program)
        mode (get (:addressings instruction) pos)
        raw-param (resolve-raw-param program instruction pos)]
    (case mode
      :immediate raw-param
      :position (get memory raw-param)
      (throw (RuntimeException. (str "Unknown mode: " mode))))))

(defn exec-bin-mem [program instruction f]
  (let [a (resolve-param program instruction 0)
        b (resolve-param program instruction 1)
        result-addr (resolve-raw-param program instruction 2)]
    (-> program
        (assoc-in [:memory result-addr] (f a b))
        (update :pointer #(+ 4 %)))))

(defn exec-read [program instruction]
  (let [result-addr (resolve-raw-param program instruction 0)
        input (:input program)]
    (if (empty? input)
      (assoc-in program [:state] :waiting-read)
      (-> program
          (update :input rest)
          (update :pointer #(+ 2 %))
          (assoc-in [:memory result-addr] (first input))))))

(defn exec-write [program instruction]
  (let [val (resolve-param program instruction 0)]
    (-> program
        (update :output #(conj % val))
        (update :pointer #(+ 2 %)))))

(defn exec-cond-jump [program instruction f]
  (let [a (resolve-param program instruction 0)
        b (resolve-param program instruction 1)]
    (if (f a)
      (assoc program :pointer b)
      (update program :pointer #(+ 3 %)))))

(defn exec-halt [program]
  (assoc program :state :halt))

(defn exec [program]
  (let [memory (:memory program)
        pointer (:pointer program)
        state (:state program)
        instruction (parse-instruction memory pointer)]
    (assert (< pointer (count memory)) (str "Out of bounds: " program))
    (if (not= state :ready) program
        (case (:op instruction)
          :sum (recur (exec-bin-mem program instruction +))
          :mul (recur (exec-bin-mem program instruction *))
          :read (recur (exec-read program instruction))
          :write (recur (exec-write program instruction))
          :jump-if-true (recur (exec-cond-jump program instruction #(not= 0 %)))
          :jump-if-false (recur (exec-cond-jump program instruction #(= 0 %)))
          :less-than (recur (exec-bin-mem program instruction #(if (< % %2) 1 0)))
          :equal (recur (exec-bin-mem program instruction #(if (= % %2) 1 0)))
          :halt (exec-halt program)
          (throw (ex-info "Unknown op in instruction" {:program program :instruction instruction}))))))

(defn map-subset? [subset superset]
  (set/subset? (set subset) (set superset)))

(deftest test-exec
  (let [exec-single-output (fn [memory input]
                             (-> (init-program {:memory memory :input input})
                                 exec
                                 (:output)
                                 last))]
    ; Day 2, part 1.
    (is (map-subset? {:memory [2,0,0,0,99] :pointer 4}
                     (exec (init-program {:memory [1,0,0,0,99] :pointer 0}))))
    (is (map-subset? {:memory [2,3,0,6,99] :pointer 4}
                     (exec (init-program {:memory [2,3,0,3,99] :pointer 0}))))
    (is (map-subset? {:memory [2,4,4,5,99,9801] :pointer 4}
                     (exec (init-program {:memory [2,4,4,5,99,0] :pointer 0}))))
    (is (map-subset? {:memory [30,1,1,4,2,5,6,0,99] :pointer 8}
                     (exec (init-program {:memory [1,1,1,4,99,5,6,0,99] :pointer 0}))))
    (is (map-subset? {:memory [3500 9 10 70 2 3 11 0 99 30 40 50] :pointer 8}
                     (exec (init-program {:memory [1 9 10 3 2 3 11 0 99 30 40 50] :pointer 0}))))

    ; Day 5, part 1.
    (is (map-subset? {:memory [1,2,3,6,99] :pointer 4}
                     (exec (init-program {:memory [1,2,3,3,99] :pointer 0}))))
    (is (map-subset? {:memory [42,0,4,0,99] :input [] :output [42] :pointer 4}
                     (exec (init-program {:memory [3,0,4,0,99] :input [42] :output [] :pointer 0}))))

    ; Day 5, part 2.
    ; Is input 8? (position mode)
    (let [memory [3,9,8,9,10,9,4,9,99,-1,8]]
      (is (= 1 (exec-single-output memory [8])))
      (is (= 0 (exec-single-output memory [7]))))
    ; Is input less than 8? (position mode)
    (let [memory [3,9,7,9,10,9,4,9,99,-1,8]]
      (is (= 1 (exec-single-output memory [7])))
      (is (= 0 (exec-single-output memory [9]))))
    ; Is input 8? (immediate mode)
    (let [memory [3,3,1108,-1,8,3,4,3,99]]
      (is (= 1 (exec-single-output memory [8])))
      (is (= 0 (exec-single-output memory [7]))))
    ; Is input less than 8? (immediate mode)
    (let [memory [3,3,1107,-1,8,3,4,3,99]]
      (is (= 1 (exec-single-output memory [7])))
      (is (= 0 (exec-single-output memory [9]))))
    ; (bool)input (position)
    (let [memory [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]]
      (is (= 0 (exec-single-output memory [0])))
      (is (= 1 (exec-single-output memory [1])))
      (is (= 1 (exec-single-output memory [2]))))
    ; (bool)input (immediate)
    (let [memory [3,3,1105,-1,9,1101,0,0,12,4,12,99,1]]
      (is (= 0 (exec-single-output memory [0])))
      (is (= 1 (exec-single-output memory [1])))
      (is (= 1 (exec-single-output memory [2]))))
    ; input < 8 => 999; input = 8 => 1000; input > 8 => 1001
    (let [memory [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,
                  98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,
                  1000,1,20,4,20,1105,1,46,98,99]]
      (is (= 999 (exec-single-output memory [7])))
      (is (= 1000 (exec-single-output memory [8])))
      (is (= 1001 (exec-single-output memory [9]))))))

(defn run-in-seq [program phase-settings]
  (loop [programs (update-in (into [] (map #(assoc program :input [%])
                                           phase-settings))
                             [0 :input]
                             #(conj % 0))
         vals []]
    (let [next-state
          (reduce
           (fn [{:keys [next-programs next-vals]} program]
             (let [program (exec (to->stdin program next-vals))]
               (assert (or
                        (= (count (:output program)) 1)
                        (= (:state program) :halt))
                       (str (:output program) (:state program)))
               {:next-programs (conj next-programs (assoc program :output []))
                :next-vals (:output program)}))
           {:next-programs [] :next-vals vals}
           programs)]
      (if (every? #(= (:state %) :halt) (:next-programs next-state))
        (last (:next-vals next-state))
        (recur (:next-programs next-state)
               (:next-vals next-state))))))

(deftest test-run-in-seq
  (is (= 139629729
         (run-in-seq (init-program
                      [3 26 1001 26 -4 26 3 27 1002 27 2 27 1 27 26 27 4 27
                       1001 28 -1 28 1005 28 6 99 0 0 5])
                     [9 8 7 6 5])))
  (is (= 18216
         (run-in-seq (init-program
                      [3 52 1001 52 -5 52 3 53 1 52 56 54 1007 54 5 55 1005 55
                       26 1001 54 -5 54 1105 1 12 1 53 54 53 1008 54 0 55 1001
                       55 1 55 2 53 55 53 4 53 1001 56 -1 56 1005 56 6 99 0 0 0
                       0 10])
                     [9 7 8 5 6]))))

(defn solve [s]
  (let [program (init-program (str->memory s))]
    (reduce max (map (partial run-in-seq program) (combo/permutations (range 5 (+ 9 1)))))))

(deftest test-solve
  (is (= 139629729
         (solve "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"))))

(defn -main
  [& _]
  (println (solve (slurp  "./large.in"))))
