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

(defn true->ins [instructions reg tmp-reg]
  (assert (not= reg tmp-reg))
  (assert (valid-output reg))
  (assert (valid-output tmp-reg))
  (-> instructions
      (append-not reg tmp-reg)
      (conj {:op :or :lhs tmp-reg :rhs reg})))

(deftest test-true->ins
  (is (= true (:J (simulate (true->ins [] :J :T) {:J false}))))
  (is (= true (:J (simulate (true->ins [] :J :T) {:J true})))))

(defn false->ins [instructions reg tmp-reg]
  (assert (not= reg tmp-reg) reg)
  (assert (valid-output reg) reg)
  (assert (valid-output tmp-reg) tmp-reg)
  (-> instructions
      (append-not reg tmp-reg)
      (conj {:op :and :lhs tmp-reg :rhs reg})))

(deftest test-false->ins
  (is (= false (:J (simulate (false->ins [] :J :T) {:J false}))))
  (is (= false (:J (simulate (false->ins [] :J :T) {:J true})))))

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

(defn conjuntion->ins [instructions reg-defs result-reg]
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

(deftest test-conjuntion->ins
  (let [run (fn [reg-defs start-state]
              (:J (simulate (conjuntion->ins [] reg-defs :J) start-state)))
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

(defn collapse-conjunction-on-reg [reg lhs rhs]
  (let [lhs-val (get lhs reg)
        rhs-val (get rhs reg)
        lhs-wo-reg (dissoc lhs reg)]
    (if (and (or (nil? lhs-val)
                 (nil? rhs-val)
                 (not= lhs-val rhs-val))
             (= lhs-wo-reg (dissoc rhs reg)))
      lhs-wo-reg
      lhs)))

(defn collapse-conjunction-on-some-reg [lhs rhs]
  (reduce
   (fn [lhs-left reg]
     (collapse-conjunction-on-reg reg lhs-left rhs))
   lhs
   (keys lhs)))

(deftest test-collapse-conjunction-on-some-reg
  (let [collapse collapse-conjunction-on-some-reg]
    (is (= {}
           (collapse {:A true} {:A false})))
    (is (= {:A true}
           (collapse {:A true} {:A true})))
    (is (= {:B false}
           (collapse {:A true :B false} {:A false :B false})))
    (is (= {:B false}
           (collapse {:A true :B false} {:A nil :B false})))
    (is (= {:B false}
           (collapse {:A true :B false} {:B false})))
    (is (= {:A true :B false}
           (collapse {:A true :B false} {:A nil :B true})))
    (is (= {:A true :B false :C true}
           (collapse {:A true :B false :C true} {:A false :B true :C true})))))

(defn find-min-ndf-ith [ndf i]
  (assoc ndf i
         (reduce collapse-conjunction-on-some-reg (nth ndf i) ndf)))

(deftest test-find-min-ndf-ith
  (is (= [{:A true :B false :C true} {:A true :C true}]
         (find-min-ndf-ith
          [{:A true :B false :C true} {:A true :B true :C true}]
          1))))

(defn find-min-ndf [ndf]
  (assert (vector? ndf))
  (loop [min-ndf ndf
         pref-min-ndf nil]
    (if (= min-ndf pref-min-ndf)
      (if (some empty? min-ndf) [] min-ndf)
      (recur
       (vec (distinct (reduce find-min-ndf-ith min-ndf (range (count min-ndf)))))
       min-ndf))))

(defn ndf->ins [result-reg tmp-reg ndf]
  (as-> [] ins
    (false->ins ins result-reg tmp-reg)
    (reduce
     (fn [acc conjuntion]
       (conj (conjuntion->ins acc conjuntion tmp-reg)
             {:op :or :lhs tmp-reg :rhs result-reg}))
     ins
     ndf)
    (vec ins)))

; ############ Other

(defn regs->ins [regs]
  (->> regs
       subsets-true-false
       comb/subsets
       (map (fn [ndf]
              (map #(assoc % :D true) ndf)))
       (map vec)
       (map find-min-ndf)
       (map #(ndf->ins :J :T %))
       (remove #(< 15 (count %)))
       distinct))

(defn solve-find-longest [ins]
  (->> ins
       (map #(vector (count %) %))
       (reduce (partial max-key first))))

(defn solve-and-print-random [program ins]
  (->> ins
       rand-nth
       (run-instructions program)
       :output
       print-output))

(defn solve-find-score [program ins]
  (->> ins
       (pmap #(run-instructions program %))
       (map :output)
       (map #(reduce max %))
       (filter #(> % 178))
       first))

(defn solve [s]
  (let [program (->> s code/str->memory code/init-program)
        regs [:A :B :C]
        ins (regs->ins regs)]
    (println "Len:" (count ins))
    (println "Longest:" (solve-find-longest ins))
    (println "Random")
    (solve-and-print-random program ins)
    (println "Solution:" (solve-find-score program ins))))

(defn -main
  [& _]
  (solve (slurp *in*)))
