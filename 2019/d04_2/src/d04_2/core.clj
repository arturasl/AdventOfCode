(ns d04-2.core
  (:gen-class)
  (:require [clojure.string :as str]))

(defn num->digits [num]
  (if (<= num 9)
    [num]
    (concat (num->digits (quot num 10)) [(mod num 10)])))

(comment
  (num->digits 123)
  (num->digits 100)
  (num->digits 0))

(defn none-decreasing? [digits]
  (apply <= digits))

(comment
  (none-decreasing? [1 2 3])
  (none-decreasing? [1 2 2])
  (none-decreasing? [1 2 1]))

(defn has-same-two-digits? [digits]
  (boolean (some #(= 2 (count %)) (partition-by identity digits))))

(comment
  (has-same-two-digits? [1 2 3])
  (has-same-two-digits? [1 2 2])
  (has-same-two-digits? [1 2 2 2])
  (has-same-two-digits? [1 1 2 2 2]))

(defn has-six-digits? [digits]
  (= (count digits) 6))

(comment
  (has-six-digits? [1 2 3])
  (has-six-digits? [1 2 3 4 5 6]))

(defn parse-rng [rng]
  (vec (->> (str/split rng #"-") (map str/trim) (map parse-long))))

(comment
  (parse-rng " 123455 - 123456 "))

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

(comment
  (solve " 123455 - 123456 "))

(defn -main
  [& _]
  (println (solve (slurp  "./large.in"))))
