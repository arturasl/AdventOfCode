(ns d21-1.core
  (:gen-class)
  (:require [intcode.core :as code]
            [clojure.string :as str]
            [clojure.test :refer [deftest is]]))

(defn print-output [program]
  (let [output (:output program)
        _ (assert (every? #(<= 10 % 127) output))
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

(defn valid-input [reg]
  (contains? #{:A :B :C :D :T :J} reg))

(defn valid-output [reg]
  (contains? #{:T :J} reg))

(defn set-true [instructions reg tmp-reg]
  (assert (not= reg tmp-reg))
  (assert (valid-output reg))
  (assert (valid-output tmp-reg))
  (-> instructions
      (conj {:op :not :lhs reg :rhs tmp-reg})
      (conj {:op :or :lhs tmp-reg :rhs reg})))

(defn set-false [instructions reg tmp-reg]
  (-> instructions
      (set-true reg tmp-reg)
      (conj {:op :not :lhs reg :rhs reg})))

(defn solve [s]
  (let [program (->> s code/str->memory code/init-program)
        program (run-instructions program (set-false [] :J :T))]
    (print-output program)))

(defn -main
  [& _]
  (solve (slurp *in*)))
