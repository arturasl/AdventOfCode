(ns d05-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]))

(defn str->program [s]
  {:memory
   (->> (str/split s #",")
        (map str/trim)
        (remove empty?)
        (map parse-long)
        (into []))
   :pointer 0
   :input []
   :output []})

(deftest test-str->program
  (is (= {:memory [2 3 0 3 99], :pointer 0 :input [] :output []}
         (str->program "2,3,0,3, 99"))))

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
    (throw (RuntimeException. (str "Unknown opcode: " opcode)))))

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
    (throw (RuntimeException. (str "Unknown op" op)))))

(defn parse-addressing [mode]
  (case mode
    \0 :position
    \1 :immediate
    (throw (RuntimeException. (str "Unknown addressing: " mode)))))

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
        val (peek (:input program))]
    (-> program
        (update :input pop)
        (update :pointer #(+ 2 %))
        (assoc-in [:memory result-addr] val))))

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

(defn exec [program]
  (let [memory (:memory program)
        pointer (:pointer program)
        instruction (parse-instruction memory pointer)]
    (case (:op instruction)
      :sum (recur (exec-bin-mem program instruction +))
      :mul (recur (exec-bin-mem program instruction *))
      :read (recur (exec-read program instruction))
      :write (recur (exec-write program instruction))
      :jump-if-true (recur (exec-cond-jump program instruction #(not= 0 %)))
      :jump-if-false (recur (exec-cond-jump program instruction #(= 0 %)))
      :less-than (recur (exec-bin-mem program instruction #(if (< % %2) 1 0)))
      :equal (recur (exec-bin-mem program instruction #(if (= % %2) 1 0)))
      :halt program
      (if (>= pointer (count memory))
        (throw (RuntimeException. "Out of bounds"))
        (throw (RuntimeException. (str "Unknown op in instruction" instruction)))))))

(defn exec-single-output [memory, input]
  (-> {:memory memory :input input :output [] :pointer 0}
      exec
      (get :output)
      last))

(deftest test-exec
  ; Day 2, part 1.
  (is (= {:memory [2,0,0,0,99] :pointer 4}
         (exec {:memory [1,0,0,0,99] :pointer 0})))
  (is (= {:memory [2,3,0,6,99] :pointer 4}
         (exec {:memory [2,3,0,3,99] :pointer 0})))
  (is (= {:memory [2,4,4,5,99,9801] :pointer 4}
         (exec {:memory [2,4,4,5,99,0] :pointer 0})))
  (is (= {:memory [30,1,1,4,2,5,6,0,99] :pointer 8}
         (exec {:memory [1,1,1,4,99,5,6,0,99] :pointer 0})))
  (is (= {:memory [3500 9 10 70 2 3 11 0 99 30 40 50] :pointer 8}
         (exec {:memory [1 9 10 3 2 3 11 0 99 30 40 50] :pointer 0})))

  ; Day 5, part 1.
  (is (= {:memory [1,2,3,6,99] :pointer 4}
         (exec {:memory [1,2,3,3,99] :pointer 0})))
  (is (= {:memory [42,0,4,0,99] :input [] :output [42] :pointer 4}
         (exec {:memory [3,0,4,0,99] :input [42] :output [] :pointer 0})))

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
    (is (= 1001 (exec-single-output memory [9])))))

(defn solve [s]
  (-> (str->program s)
      (assoc :input [5])
      exec
      (get :output)
      last))

(defn -main
  [& _]
  (println (solve (slurp  "./large.in"))))
