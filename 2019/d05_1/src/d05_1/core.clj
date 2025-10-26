(ns d05-1.core
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

(defn read-memory [memory {:keys [address mode]}]
  (case mode
    :immediate address
    :indirect (recur memory {:address (get memory address) :mode :immediate})
    (throw (RuntimeException. (str "Unknown mode: " mode)))))

(deftest test-read-memory
  (is (= 1 (read-memory [1 2 3] {:address 1 :mode :immediate})))
  (is (= 2 (read-memory [1 2 3] {:address 1 :mode :indirect}))))

(defn opcode->op [opcode]
  (case (mod opcode 100)
    1 :sum
    2 :mul
    3 :read
    4 :write
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
    :halt 0
    (throw (RuntimeException. (str "Unknown op" op)))))

(defn parse-addressing [mode]
  (case mode
    \0 :indirect
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
  (is (= {:op :mul :addressings [:indirect :immediate :indirect] :pointer 0 :num-params 3}
         (parse-instruction [1002 1 1 1] 0)))
  (is (= {:op :mul :addressings [:indirect :indirect :indirect] :pointer 0 :num-params 3}
         (parse-instruction [2 1 1 1] 0))))

(defn resolve-params [memory instruction]
  (let [raw-params (subvec memory
                           (+ (:pointer instruction) 1)
                           (+ (:pointer instruction) (:num-params instruction) 1))]
    (vec (map #(read-memory memory {:address % :mode %2}) raw-params (:addressings instruction)))))

(defn exec-bin-mem [program instruction f]
  (let [params (resolve-params (:memory program) instruction)
        addr (get (:memory program) (+ 3 (:pointer instruction)))]
    (-> program
        (assoc-in [:memory addr] (apply f (subvec params 0 2)))
        (update :pointer #(+ 4 %)))))

(defn exec-read [program instruction]
  (let [addr (get (:memory program) (+ 1 (:pointer instruction)))
        val (peek (:input program))]
    (-> program
        (update :input pop)
        (update :pointer #(+ 2 %))
        (assoc-in [:memory addr] val))))

(defn exec-write [program instruction]
  (let [params (resolve-params (:memory program) instruction)]
    (-> program
        (update :output #(conj % (last params)))
        (update :pointer #(+ 2 %)))))

(defn exec [program]
  (let [memory (:memory program)
        pointer (:pointer program)
        instruction (parse-instruction memory pointer)]
    (case (:op instruction)
      :sum (recur (exec-bin-mem program instruction +))
      :mul (recur (exec-bin-mem program instruction *))
      :read (recur (exec-read program instruction))
      :write (recur (exec-write program instruction))
      :halt program
      (if (>= pointer (count memory))
        (throw (RuntimeException. "Out of bounds"))
        (throw (RuntimeException. (str "Unknown op in instruction" instruction)))))))

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
         (exec {:memory [3,0,4,0,99] :input [42] :output [] :pointer 0}))))

(defn solve [s]
  (-> (str->program s)
      (assoc :input [1])
      exec
      (get :output)
      last))

(defn -main
  [& _]
  (println (solve (slurp  "./large.in"))))
