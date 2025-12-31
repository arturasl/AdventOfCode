(ns d21-1.core
  (:gen-class)
  (:require [intcode.core :as code]
            [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [clojure.math.combinatorics :as comb]))

; ############ Utils

(defn subsets [els]
  (map #(into #{} %)
       (comb/subsets (vec els))))

(defn subsets-true-false [regs]
  (vec
   (map
    (fn [sub]
      (into {} (map #(vector % (contains? sub %))) regs))
    (subsets regs))))

; ############ Simulate

(defn simulate [instructions state]
  (reduce
   (fn [acc-state {:keys [op lhs rhs]}]
     (let [lhs-val (get acc-state lhs)
           rhs-val (get acc-state rhs)]
       (case op
         :or (assoc acc-state rhs (or lhs-val rhs-val))
         :and (assoc acc-state rhs (and lhs-val rhs-val))
         :not (assoc acc-state rhs (not lhs-val)))))
   state
   instructions))

(deftest test-simulate
  (is (= {:A true :B true}
         (simulate [{:op :or :lhs :A :rhs :B}] {:A true :B false}))))

; ############ Run

(defn print-output [output]
  (let [_ (assert (every? #(<= 10 % 127) output))
        output-str (apply str (map char output))]
    (println output-str)))

(defn ins->str [{:keys [op lhs rhs]}]
  (str (str/upper-case (name op)) \space (name lhs) \space (name rhs)))

(deftest test-ins->str
  (is (= "NOT A J"
         (ins->str {:op :not :lhs :A :rhs :J}))))

(defn run-instructions [program instructions]
  (assert (vector? instructions))
  (let [str-instructions (vec (map ins->str instructions))
        with-end (conj str-instructions "WALK" "")
        bytes (vec (map int (str/join \newline with-end)))]
    (code/exec (code/to->stdin program bytes))))

; ############ Simple instructions

(defn valid-input [reg]
  (contains? #{:A :B :C :D :T :J} reg))

(defn valid-output [reg]
  (contains? #{:T :J} reg))

(defn append-not [instruction reg result-reg]
  (assert (vector? instruction))
  (let [to-add {:op :not :lhs reg :rhs result-reg}]
    (if (= (last instruction) to-add)
      (pop instruction)
      (conj instruction to-add))))

(defn set-true [instructions reg tmp-reg]
  (assert (not= reg tmp-reg))
  (assert (valid-output reg))
  (assert (valid-output tmp-reg))
  (-> instructions
      (append-not reg tmp-reg)
      (conj {:op :or :lhs tmp-reg :rhs reg})))

(deftest test-set-true
  (is (= true (:J (simulate (set-true [] :J :T) {:J false}))))
  (is (= true (:J (simulate (set-true [] :J :T) {:J true})))))

(defn set-false [instructions reg tmp-reg]
  (assert (not= reg tmp-reg) reg)
  (assert (valid-output reg) reg)
  (assert (valid-output tmp-reg) tmp-reg)
  (-> instructions
      (append-not reg tmp-reg)
      (conj {:op :and :lhs tmp-reg :rhs reg})))

(deftest test-set-false
  (is (= false (:J (simulate (set-false [] :J :T) {:J false}))))
  (is (= false (:J (simulate (set-false [] :J :T) {:J true})))))

(defn set-to [instructions input-reg result-reg]
  (assert (valid-input input-reg) input-reg)
  (assert (valid-output result-reg) result-reg)
  (-> instructions
      (append-not input-reg result-reg)
      (append-not result-reg result-reg)))

(deftest test-set-to
  (is (= false (:J (simulate (set-to [] :T :J) {:T false}))))
  (is (= true (:J (simulate (set-to [] :T :J) {:T true})))))

; ############ Complex instructions

(defn ensure-conjunctive [instructions reg-defs result-reg]
  (assert (not-empty reg-defs))
  (assert (every? valid-input (keys reg-defs))
          (str reg-defs))
  (assert (valid-output result-reg))
  (let [reg-defs (sort-by second reg-defs)
        [first-reg first-incl] (first reg-defs)
        rest-defs (rest reg-defs)]
    (as-> instructions ins
      (set-to ins first-reg result-reg)
      (if (not first-incl)
        (append-not ins result-reg result-reg)
        ins)
      (reduce (fn [acc [reg incl]]
                (if incl
                  (conj acc
                        {:op :and :lhs reg :rhs result-reg})
                  (-> acc
                      (append-not result-reg result-reg)
                      (conj {:op :or :lhs reg :rhs result-reg})
                      (append-not result-reg result-reg))))
              ins
              rest-defs)
      (vec ins))))

(deftest test-ensure-conjunctive
  (let [run (fn [reg-defs start-state]
              (:J (simulate (ensure-conjunctive [] reg-defs :J) start-state)))
        find-true-states (fn [reg-defs]
                           (let [all-regs (keys reg-defs)
                                 initial-states (subsets-true-false all-regs)
                                 true-states (for [init-state initial-states
                                                   :let [result (run reg-defs init-state)]
                                                   :when result]
                                               init-state)]
                             true-states))
        test-regs (fn [regs]
                    (doseq [reg-def (subsets-true-false regs)]
                      (is (= [reg-def]
                             (find-true-states reg-def)))))]
    (test-regs [:A])
    (test-regs [:A :B])
    (test-regs [:A :B :C])
    (test-regs [:A :B :C :D])))

(defn ensure-any-true [result-reg tmp-reg vec-reg-defs]
  (as-> [] ins
    (set-false ins result-reg tmp-reg)
    (reduce
     (fn [acc reg-defs]
       (conj (ensure-conjunctive acc reg-defs tmp-reg)
             {:op :or :lhs tmp-reg :rhs result-reg}))
     ins
     vec-reg-defs)
    (vec ins)))

; ############ Other

(defn are-instructions-equiv [lhs rhs]
  (let [used-regs (into #{}
                        (concat
                         (map :lhs lhs)
                         (map :rhs lhs)
                         (map :lhs rhs)
                         (map :rhs rhs)))]
    (every?
     (fn [state]
       (= (simulate lhs state)
          (simulate rhs state)))
     (subsets-true-false used-regs))))

(deftest test-are-instructions-equiv
  (is (are-instructions-equiv
       [{:op :not :lhs :J :rhs :J} {:op :not :lhs :J :rhs :J}]
       []))
  (is (not (are-instructions-equiv
            [{:op :not :lhs :J :rhs :J} {:op :not :lhs :J :rhs :J}]
            [{:op :not :lhs :J :rhs :J}])))
  (is (are-instructions-equiv
       [{:op :not :lhs :J :rhs :J} {:op :not :lhs :J :rhs :J}]
       [])))

(defn run-vec-reg-defs [program vec-reg-defs]
  (let [instructions (ensure-any-true :J :T (vec vec-reg-defs))
        output (:output (run-instructions program instructions))]
    output))

(defn solve [s]
  (let [program (->> s code/str->memory code/init-program)
        input-regs [:A :B :C :D]]
    ; (->> input-regs
    ;      subsets-true-false
    ;      comb/subsets
    ;      (map #(ensure-any-true :J :T %))
    ;      (map count)
    ;      (reduce max))))

    ; Out of memory; at most 15 instructions can be stored
    ; (->> input-regs
    ;      subsets-true-false
    ;      comb/subsets
    ;      rand-nth
    ;      (run-vec-reg-defs program)
    ;      print-output)))

    (->> input-regs
         subsets-true-false
         comb/subsets
         (pmap #(reduce max (run-vec-reg-defs program %)))
         (filter #(> % 178))
         first)))

(defn -main
  [& _]
  (println (solve (slurp *in*))))
