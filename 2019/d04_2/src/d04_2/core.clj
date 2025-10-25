(ns d04-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]))

(defn num->digits [num]
  (if (<= num 9)
    [num]
    (concat (num->digits (quot num 10)) [(mod num 10)])))

(deftest test-num->digits
  (is (= [1 2 3] (num->digits 123)))
  (is (= [1 0 0] (num->digits 100)))
  (is (= [0] (num->digits 0))))

(defn none-decreasing? [digits]
  (apply <= digits))

(deftest test-none-decreasing?
  (is (true? (none-decreasing? [1 2 3])))
  (is (true? (none-decreasing? [1 2 2])))
  (is (false? (none-decreasing? [1 2 1]))))

(defn has-same-two-digits? [digits]
  (boolean (some #(= 2 (count %)) (partition-by identity digits))))

(deftest test-has-same-two-digits?
  (is (false? (has-same-two-digits? [1 2 3])))
  (is (true? (has-same-two-digits? [1 2 2])))
  (is (false? (has-same-two-digits? [1 2 2 2])))
  (is (true? (has-same-two-digits? [1 1 2 2 2]))))

(defn has-six-digits? [digits]
  (= (count digits) 6))

(deftest test-has-six-digits?
  (is (false? (has-six-digits? [1 2 3])))
  (is (true? (has-six-digits? [1 2 3 4 5 6]))))

(defn parse-rng [rng]
  (vec (->> (str/split rng #"-") (map str/trim) (map parse-long))))

(deftest test-parse-rng
  (is (= [123455 123456] (parse-rng " 123455 - 123456 "))))

(defn solve [rng]
  (let [[low high] (parse-rng rng)]
    (->> (range low (+ high 1))
         (map (fn [num]
                (let [digits (num->digits num)]
                  (and (none-decreasing? digits)
                       (has-same-two-digits? digits)
                       (has-six-digits? digits)))))
         (filter true?)
         count)))

(deftest test-solve
  (is (= 1 (solve " 123455 - 123456 "))))

(defn -main
  [& _]
  (println (solve (slurp  "./large.in"))))
