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
  (->> (str/split-lines s)
       (map str/trim)
       (remove empty?)
       (map str->reaction)))

(defn applicable? [reaction have]
  (let [needs (:needs reaction)
        needed-have-after (into {} (map
                                    #(vector % ((if (= % :ORE) + -) (get have % 0) (get needs %)))
                                    (keys needs)))
        all-positive (every? (partial <= 0) (vals needed-have-after))]
    (when all-positive
      (into {}
            (remove #(zero? (second %))
                    (merge-with
                     +
                     (merge have needed-have-after)
                     {(:ingredient reaction) (:cnt reaction)}))))))

(deftest test-applicable?
  (is (= nil
         (applicable? {:ingredient :AB :cnt 5 :needs {:A 1 :B 2 :ORE 9}} {:A 8 :C 3})))
  (is (= {:A 7 :C 3 :ORE 9 :AB 5}
         (applicable? {:ingredient :AB :cnt 5 :needs {:A 1 :B 2 :ORE 9}} {:A 8 :B 2 :C 3})))
  (is (= {:AB 9}
         (applicable? {:ingredient :AB :cnt 5 :needs {:AB 3}} {:AB 7}))))

(defn minimize-ore
  ([reactions] (minimize-ore (priority-map {} 0) reactions #{} 0))
  ([queue reactions visited its]
   (let [[have ore] (peek queue)
         queue (pop queue)]
     (when (= (mod its 10000) 0)
       (println "its: " its
                ", states visited: " (count visited)
                ", have: " have
                ", ore " ore))
     (if (:FUEL have)
       ore
       (let [applicable-haves (remove nil? (map #(applicable? % have) reactions))
             sorted-haves (sort-by :ORE applicable-haves)
             {:keys [next-queue next-visited]}
             (reduce (fn [{:keys [next-queue next-visited]} applicable-have]
                       (let [have-wo-ore (dissoc applicable-have :ORE)]
                         (if (contains? next-visited have-wo-ore)
                           {:next-queue next-queue :next-visited next-visited}
                           {:next-queue (assoc next-queue applicable-have (:ORE applicable-have))
                            :next-visited (conj next-visited have-wo-ore)})))
                     {:next-queue queue :next-visited visited}
                     sorted-haves)]
         (recur next-queue reactions next-visited (inc its)))))))

(deftest test-minimize-ore
  (let [str->minimize-ore (comp minimize-ore str->reactions (partial str/join "\n"))]
    (is (= 31
           (str->minimize-ore ["10 ORE => 10 A"
                               "1 ORE => 1 B"
                               "7 A, 1 B => 1 C"
                               "7 A, 1 C => 1 D"
                               "7 A, 1 D => 1 E"
                               "7 A, 1 E => 1 FUEL"])))
    (is (= 13312
           (str->minimize-ore ["157 ORE => 5 NZVS"
                               "165 ORE => 6 DCFZ"
                               "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL"
                               "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ"
                               "179 ORE => 7 PSHF"
                               "177 ORE => 5 HKGWZ"
                               "7 DCFZ, 7 PSHF => 2 XJWVT"
                               "165 ORE => 2 GPVTF"
                               "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"])))))

    ; (is (= 180697
    ;        (str->minimize-ore ["2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG"
    ;                            "17 NVRVD, 3 JNWZP => 8 VPVL"
    ;                            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL"
    ;                            "22 VJHF, 37 MNCFX => 5 FWMGM"
    ;                            "139 ORE => 4 NVRVD"
    ;                            "144 ORE => 7 JNWZP"
    ;                            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC"
    ;                            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV"
    ;                            "145 ORE => 6 MNCFX"
    ;                            "1 NVRVD => 8 CXFTF"
    ;                            "1 VJHF, 6 MNCFX => 4 RFSQX"
    ;                            "176 ORE => 6 VJHF"])))

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
  (println
   (minimize-ore
    (str->reactions "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL"))))
