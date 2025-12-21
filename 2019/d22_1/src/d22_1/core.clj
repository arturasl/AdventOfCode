(ns d22-1.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]))

(defn str->op [s]
  (let [deal (re-matches #"^deal with increment (\d+)$" s)
        cut (re-matches #"^cut (-?\d+)$" s)]
    (cond
      (not (nil? deal)) {:op :deal :amount (parse-long (second deal))}
      (not (nil? cut)) {:op :cut :amount (parse-long (second cut))}
      :else (do
              (assert (= s "deal into new stack"))
              {:op :reverse}))))

(deftest test-str->op
  (is (= {:op :deal :amount 7}
         (str->op "deal with increment 7")))
  (is (= {:op :deal :amount 78}
         (str->op "deal with increment 78")))
  (is (= {:op :cut :amount -1}
         (str->op "cut -1")))
  (is (= {:op :cut :amount 12}
         (str->op "cut 12")))
  (is (= {:op :reverse}
         (str->op "deal into new stack"))))

(defn str->ops [s]
  (->> s
       (str/split-lines)
       (map str/trim)
       (remove empty?)
       (map str->op)))

(defn follow [pos total ops]
  (reduce
   (fn [cur-pos {:keys [op amount]}]
     (case op
       :reverse (- total cur-pos 1)
       :cut (let [amount (if (<= 0 amount) amount (+ total amount))]
              (if (< cur-pos amount)
                (+ (- total amount) cur-pos)
                (- cur-pos amount)))
       :deal (mod (* cur-pos amount) total)
       (throw (ex-info "Unknown op" {:op op}))))
   pos
   ops))

(deftest test-follow
  ;
  (is (= 9 (follow 0 10 [{:op :reverse}])))
  (is (= 8 (follow 1 10 [{:op :reverse}])))
  (is (= 0 (follow 9 10 [{:op :reverse}])))
  ;
  (is (= 7 (follow 0 10 [{:op :cut :amount 3}])))
  (is (= 8 (follow 1 10 [{:op :cut :amount 3}])))
  (is (= 9 (follow 2 10 [{:op :cut :amount 3}])))
  (is (= 0 (follow 3 10 [{:op :cut :amount 3}])))
  (is (= 1 (follow 4 10 [{:op :cut :amount 3}])))
  ;
  (is (= 4 (follow 0 10 [{:op :cut :amount -4}])))
  (is (= 9 (follow 5 10 [{:op :cut :amount -4}])))
  (is (= 0 (follow 6 10 [{:op :cut :amount -4}])))
  ;
  (is (= 0 (follow 0 10 [{:op :deal :amount 3}])))
  (is (= 3 (follow 1 10 [{:op :deal :amount 3}])))
  (is (= 1 (follow 7 10 [{:op :deal :amount 3}])))
  (is (= 2 (follow 4 10 [{:op :deal :amount 3}]))))

(defn solve [s]
  (let [ops (str->ops s)]
    (follow 2019 10007 ops)))

(defn -main
  [& _]
  (println (solve (slurp *in*))))
