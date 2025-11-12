(ns d14-1.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [clojure.data.priority-map :refer [priority-map]]))

(defn match->ingredient [[_ s_quantity ingredient]]
  {:ingredient (keyword ingredient) :cnt (parse-long s_quantity)})

(defn str->reaction [s]
  (let [[inp out] (str/split s #"=>")
        needs (into {} (map #(vector (:ingredient %) (:cnt %)) (map match->ingredient (re-seq #"(\d+) (\w+)" inp))))
        {:keys [ingredient cnt]} (match->ingredient (re-find #"(\d+) (\w+)" out))]
    (assert (or (not= ingredient :FUEL) (= 1 cnt))
            "Rule that produce FUEL, should produce single FUEL")
    (assert (not= ingredient :ORE)
            "No rule should produce ORE")
    (assert (or (not (contains? needs :ORE)) (= (count needs) 1))
            "Rule that requires ORE, should only require ORE and no other ingredients")
    (assert (= (count needs) (inc (count (re-seq #"," inp))))
            "All requirements should be unique")
    {:ingredient ingredient :cnt cnt :needs needs}))

(deftest test-str->reaction
  (is (= {:ingredient :AB  :cnt 1 :needs {:AZ 32 :B 4}}
         (str->reaction "32 AZ, 4 B => 1 AB"))))

(defn str->reactions [s]
  (->> (if (string? s) (str/split-lines s) s)
       (map str/trim)
       (remove empty?)
       (map str->reaction)))

(defn collapse-rule [rule target]
  (let [rule-produces (:ingredient rule)
        rule-produces-cnt (:cnt rule)
        target-needs-rule-outputs (rule-produces (:needs target))]
    (if (or (nil? target-needs-rule-outputs) (not= (mod target-needs-rule-outputs rule-produces-cnt) 0))
      target
      (assoc target :needs
             (merge-with +
                         (dissoc (:needs target) rule-produces)
                         (into {} (map (fn [[ingredient cnt]] [ingredient (* cnt (/ target-needs-rule-outputs rule-produces-cnt))]) (:needs rule))))))))

(deftest test-collapse-rule
  ; Not multiplicative.
  (is (= {:ingredient :B :cnt 6 :needs {:A 11 :B 1 :D 3}}
         (collapse-rule
          {:ingredient :A :cnt 5 :needs {:B 1 :C 2}}
          {:ingredient :B :cnt 6 :needs {:A 11 :B 1 :D 3}})))
  ; Does not need.
  (is (= {:ingredient :B :cnt 6 :needs {:Z 11 :B 1 :D 3}}
         (collapse-rule
          {:ingredient :A :cnt 5 :needs {:B 1 :C 2}}
          {:ingredient :B :cnt 6 :needs {:Z 11 :B 1 :D 3}})))
  ; Ok.
  (is (= {:ingredient :B :cnt 6 :needs {:B 3 :C 4 :D 3 :E 8}}
         (collapse-rule
          {:ingredient :A :cnt 5 :needs {:B 1 :C 2 :E 4}}
          {:ingredient :B :cnt 6 :needs {:A 10 :B 1 :D 3}}))))

(defn collapse-rules [rule rules]
  (loop [result-rules rules]
    (let [next-result (map (partial collapse-rule rule) result-rules)]
      (if (= result-rules next-result)
        next-result
        (recur next-result)))))

(defn collapse-all-rules [rules]
  (loop [result-rules rules]
    (let [next-result (reduce (fn [acc-rules rule] (collapse-rules rule acc-rules)) result-rules result-rules)]
      (if (= result-rules next-result)
        next-result
        (recur next-result)))))

(defn remove-unused [rules]
  (loop [result-rules rules]
    (let [all-needs (reduce (fn [acc rule] (into acc (keys (:needs rule)))) #{:FUEL} result-rules)
          next-rules (filter #(contains? all-needs (:ingredient %)) result-rules)]
      (if (= result-rules next-rules)
        next-rules
        (recur next-rules)))))

(defn clean-rules [rules]
  (->> rules
       collapse-all-rules
       remove-unused))

(deftest test-clean-rules
  (is (= [{:ingredient :B, :cnt 3, :needs {:ORE 8}}
          {:ingredient :C, :cnt 5, :needs {:ORE 7}}
          {:ingredient :FUEL, :cnt 1, :needs {:B 23, :C 37, :ORE 45}}]
         (->> ["9 ORE => 2 A"
               "8 ORE => 3 B"
               "7 ORE => 5 C"
               "3 A, 4 B => 1 AB"
               "5 B, 7 C => 1 BC"
               "4 C, 1 A => 1 CA"
               "2 AB, 3 BC, 4 CA => 1 FUEL"]
              str->reactions
              clean-rules))))

(defn get-ingredient-maxes [rules]
  (merge-with +
              (into {} (map #(vector (:ingredient %) (:cnt %)) rules))
              (apply (partial merge-with max) (map :needs rules))))

(deftest test-get-ingredient-maxes
  (is (= {:C 17 :D 3}
         (get-ingredient-maxes [{:ingredient :C, :cnt 5, :needs {:C 9 :D 3}}
                                {:ingredient :C, :cnt 5, :needs {:C 12 :D 3}}])))
  (is (= {:ORE 6}
         (get-ingredient-maxes [{:ingredient :ORE, :cnt 5, :needs {:ORE 1}}]))))

(defn try-apply [reaction have]
  (let [needs (:needs reaction)
        have-after-needs (into {} (map (fn [[need-ingredient need-cnt]]
                                         (vector need-ingredient (- (get have need-ingredient 0) need-cnt)))
                                       needs))
        applied (merge-with +
                            (merge have have-after-needs)
                            {(:ingredient reaction) (:cnt reaction)})
        used-ore (if (neg? (:ORE applied 0)) (abs (:ORE applied)) 0)
        applied-fixing-ore (if (:ORE applied) (update applied :ORE (partial max 0)) applied)
        all-positive (every? (partial <= 0) (vals applied-fixing-ore))]
    (when all-positive
      {:used-ore used-ore
       :next-have (into {} (remove #(zero? (second %)) applied-fixing-ore))})))

(deftest test-try-apply
  (is (= nil
         (try-apply {:ingredient :AB :cnt 5 :needs {:A 1 :B 2 :ORE 9}} {:A 8 :C 3})))
  (is (= {:used-ore 0 :next-have {:A 7 :C 3 :AB 5}}
         (try-apply {:ingredient :AB :cnt 5 :needs {:A 1 :B 2}} {:A 8 :B 2 :C 3})))
  (is (= {:used-ore 0 :next-have {:AB 5 :ORE 1}}
         (try-apply {:ingredient :AB :cnt 5 :needs {:ORE 3}} {:ORE 4})))
  (is (= {:used-ore 0 :next-have {:AB 5}}
         (try-apply {:ingredient :AB :cnt 5 :needs {:ORE 3}} {:ORE 3})))
  (is (= {:used-ore 1 :next-have {:AB 5}}
         (try-apply {:ingredient :AB :cnt 5 :needs {:ORE 4}} {:ORE 3}))))

(defn next-state [cur-state {:keys [used-ore next-have]}]
  (-> cur-state
      (assoc :have next-have)
      (update :used-ore (partial + used-ore))))

(defn next-globals [cur-globals cur-state]
  (assoc-in cur-globals [:arivals (:have cur-state)] (:used-ore cur-state)))

(defn sort-applicables [applicables]
  applicables)
  ; (shuffle applicables))
  ; (sort-by #(vector (- (count (:next-have %))) (reduce + (vals (:next-have %))) (rand)) applicables))

(def ^:const max-ore-ever 1000000000)

(defn init-search-space []
  [{:have {} :used-ore 0}])
(defn pop-search-space [search-space] (pop search-space))
(defn peek-search-space [search-space] (peek search-space))
(defn push-all-search-space [search-space coll]
  (apply conj search-space coll))

; (defn priority-search-space-ord [key]
;   ; (vector (- (count (:have key)))
;   ;         (reduce + (vals (:have key)))
;   ;         (rand)))
;   (vector (:used-ore key) (rand)))
; (defn init-search-space []
;   (let [key {:have {} :used-ore 0}]
;     (priority-map key (priority-search-space-ord key))))
; (defn pop-search-space [search-space] (pop search-space))
; (defn peek-search-space [search-space] (first (peek search-space)))
; (defn push-all-search-space [search-space coll]
;   (into search-space (map #(vector % (priority-search-space-ord %)) coll)))

(defn minimize-ore-s
  ([init-reactions]
   (let [reactions (clean-rules init-reactions)
         ingredient-maxes (get-ingredient-maxes reactions)
         {:keys [:its :fuel-required-ore]} (minimize-ore-s reactions ingredient-maxes)]
     (println "Finished in its:" its "with result:" fuel-required-ore)
     fuel-required-ore))
  ([reactions ingredient-maxes]
   (loop [search-space (init-search-space)
          globals {:its 1 :arivals {} :fuel-required-ore max-ore-ever}]
     (if (empty? search-space) globals
         (let [{:keys [:used-ore :have] :as state} (peek-search-space search-space)
               search-space (pop-search-space search-space)
               globals (update globals :its inc)]
           (when (zero? (mod (:its globals) 100000))
             (println "Cur globals" (dissoc globals :arivals)
                      "search-space size:" (count search-space)
                      "now:" state))
           (cond
             (or
              (>= used-ore (:fuel-required-ore globals))
              (some #(< (% ingredient-maxes) (% have)) (keys have))
              (<= (get (:arivals globals) have max-ore-ever) used-ore))
             (recur search-space globals)
             (:FUEL have) (recur search-space (assoc globals :fuel-required-ore used-ore))
             :else (let [applicable (remove nil? (map #(try-apply % have) reactions))
                         sorted-applicables (sort-applicables applicable)
                         next-globals (next-globals globals state)
                         next-search-space (push-all-search-space
                                            search-space
                                            (map (partial next-state state) sorted-applicables))]
                     (recur next-search-space next-globals))))))))

(deftest test-minimize-ore
  (let [str->minimize-ore (comp minimize-ore-s str->reactions)]
    (is (= 31
           (str->minimize-ore ["10 ORE => 10 A"
                               "1 ORE => 1 B"
                               "7 A, 1 B => 1 C"
                               "7 A, 1 C => 1 D"
                               "7 A, 1 D => 1 E"
                               "7 A, 1 E => 1 FUEL"])))

    (is (= 165
           (str->minimize-ore ["9 ORE => 2 A"
                               "8 ORE => 3 B"
                               "7 ORE => 5 C"
                               "3 A, 4 B => 1 AB"
                               "5 B, 7 C => 1 BC"
                               "4 C, 1 A => 1 CA"
                               "2 AB, 3 BC, 4 CA => 1 FUEL"])))

    (is (= 13312
           (str->minimize-ore ["157 ORE => 5 NZVS"
                               "165 ORE => 6 DCFZ"
                               "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL"
                               "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ"
                               "179 ORE => 7 PSHF"
                               "177 ORE => 5 HKGWZ"
                               "7 DCFZ, 7 PSHF => 2 XJWVT"
                               "165 ORE => 2 GPVTF"
                               "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"])))

    (is (= 180697
           (str->minimize-ore ["2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG"
                               "17 NVRVD, 3 JNWZP => 8 VPVL"
                               "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL"
                               "22 VJHF, 37 MNCFX => 5 FWMGM"
                               "139 ORE => 4 NVRVD"
                               "144 ORE => 7 JNWZP"
                               "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC"
                               "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV"
                               "145 ORE => 6 MNCFX"
                               "1 NVRVD => 8 CXFTF"
                               "1 VJHF, 6 MNCFX => 4 RFSQX"
                               "176 ORE => 6 VJHF"])))))

    ; (is (= 2210736
    ;        (str->minimize-ore ["171 ORE => 8 CNZTR"
    ;                            "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL"
    ;                            "114 ORE => 4 BHXH"
    ;                            "14 VRPVC => 6 BMBT"
    ;                            "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL"
    ;                            "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT"
    ;                            "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW"
    ;                            "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW"
    ;                            "5 BMBT => 4 WPTQ"
    ;                            "189 ORE => 9 KTJDG"
    ;                            "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP"
    ;                            "12 VRPVC, 27 CNZTR => 2 XDBXC"
    ;                            "15 KTJDG, 12 BHXH => 5 XCVML"
    ;                            "3 BHXH, 2 VRPVC => 7 MZWV"
    ;                            "121 ORE => 7 VRPVC"
    ;                            "7 XCVML => 6 RJRHP"
    ;                            "5 BHXH, 4 VRPVC => 5 LTCX"])))))

(defn -main [& _]
  (-> (slurp "./large.in")
      str->reactions
      minimize-ore-s
      println))
