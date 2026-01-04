(ns d14-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]))

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
       (map str->reaction)
       (group-by :ingredient)
       (map (fn [[ingredient recepies]]
              (assert (= (count recepies) 1))
              (vector ingredient (first recepies))))
       (into {})))

(defn ceildiv [a b]
  (+ (quot a b)
     (if (zero? (mod a b)) 0 1)))

(defn minimize-ore-s [init-reactions need-fuel]
  (loop [need {:FUEL need-fuel}
         have {}
         ore 0]
    (if (empty? need) ore
        (let [[need-ing need-cnt] (first need)
              already-have-cnt (min need-cnt (get have need-ing 0))
              still-need-cnt (- need-cnt already-have-cnt)
              recipe (get init-reactions need-ing)
              run-recipe-times (ceildiv still-need-cnt (:cnt recipe))
              leftovers-cnt (- (* run-recipe-times (:cnt recipe)) still-need-cnt)
              next-have (assoc have need-ing (+ (- (get have need-ing 0) already-have-cnt) leftovers-cnt))
              will-produce (update-vals
                            (:needs recipe)
                            (fn [produce-cnt] (* produce-cnt run-recipe-times)))
              next-need (merge-with +
                                    (into {} (rest need))
                                    will-produce)
              next-ore (+ ore (:ORE next-need 0))]
          (recur (dissoc next-need :ORE) next-have next-ore)))))

(defn fuel-with-fixed-ore
  ([init-reactions]
   (fuel-with-fixed-ore init-reactions 1000000000000))
  ([init-reactions target]
   (loop [mi 0
          ma 1000000000]
     (if (>= mi ma) (dec mi)
         (let [mid (+ mi (quot (- ma mi) 2))
               need-ore (minimize-ore-s init-reactions mid)]
           (if (< need-ore target)
             (recur (inc mid) ma)
             (recur mi mid)))))))

(deftest test-fuel-with-fixed-ore
  (let [run (comp fuel-with-fixed-ore str->reactions)]
    (is (= 82892753
           (run ["157 ORE => 5 NZVS"
                 "165 ORE => 6 DCFZ"
                 "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL"
                 "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ"
                 "179 ORE => 7 PSHF"
                 "177 ORE => 5 HKGWZ"
                 "7 DCFZ, 7 PSHF => 2 XJWVT"
                 "165 ORE => 2 GPVTF"
                 "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"])))

    (is (= 5586022
           (run ["2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG"
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
                 "176 ORE => 6 VJHF"])))

    (is (= 460664
           (run ["171 ORE => 8 CNZTR"
                 "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL"
                 "114 ORE => 4 BHXH"
                 "14 VRPVC => 6 BMBT"
                 "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL"
                 "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT"
                 "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW"
                 "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW"
                 "5 BMBT => 4 WPTQ"
                 "189 ORE => 9 KTJDG"
                 "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP"
                 "12 VRPVC, 27 CNZTR => 2 XDBXC"
                 "15 KTJDG, 12 BHXH => 5 XCVML"
                 "3 BHXH, 2 VRPVC => 7 MZWV"
                 "121 ORE => 7 VRPVC"
                 "7 XCVML => 6 RJRHP"
                 "5 BHXH, 4 VRPVC => 5 LTCX"])))))

(defn -main [& _]
  (-> (slurp *in*)
      str->reactions
      fuel-with-fixed-ore
      println))
