(ns d16-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]))

(defn str->num-vec [s]
  (vec (map #(Character/digit %1 10) (str/trim s))))

(defn run-its [initial its]
  (assert (vector? initial))
  (let [idx-range (reverse (range (dec (count initial))))]
    (loop [its its
           prev initial]
      (if (zero? its) prev
          (recur (dec its)
                 (vec (reduce
                       (fn [acc idx]
                         (conj acc (mod (+ (peek acc) (get prev idx)) 10)))
                       (list (last prev))
                       idx-range)))))))

(deftest test-run-its
  (let [run-str (fn [its]
                  (apply str (run-its (str->num-vec "5678") its)))]
    (is (= "5678" (run-str 0)))
    (is (= "6158" (run-str 1)))
    (is (= "0438" (run-str 2)))
    (is (= "5518" (run-str 3)))
    (is (= "9498" (run-str 4)))))

(defn get-offset [nums]
  (reduce (fn [acc digit] (+ (* acc 10) digit))
          0
          (take 7 nums)))

(deftest test-get-offset
  (let [str->get-offset (fn [s] (get-offset (str->num-vec s)))]
    (is (= 303673 (str->get-offset "03036732577212944063491565474664")))
    (is (= 293510 (str->get-offset "02935109699940807407585447034323")))
    (is (= 308177 (str->get-offset "03081770884921959731165446850517")))))

(defn at-offset-with-reps [nums offset reps]
  (let [len (count nums)
        offset-in-first (mod offset len)
        reps-left (- reps (quot offset len))
        _ (assert (>= reps-left 1))]
    (vec (drop offset-in-first (flatten (repeat reps-left nums))))))

(deftest test-at-offset-with-reps
  (let [str->at-offset-with-reps (fn [offset reps] (apply str (at-offset-with-reps (str->num-vec "0123456789") offset reps)))]
    (is (= "0123456789" (str->at-offset-with-reps 0 1)))
    (is (= "01234567890123456789" (str->at-offset-with-reps 0 2)))
    (is (= "1234567890123456789" (str->at-offset-with-reps 1 2)))
    (is (= "34567890123456789" (str->at-offset-with-reps 3 2)))
    (is (= "0123456789" (str->at-offset-with-reps 10 2)))
    (is (= "90123456789" (str->at-offset-with-reps 9 2)))
    (is (= "123456789" (str->at-offset-with-reps 11 2)))))

(defn solve [s]
  (let [nums (str->num-vec s)
        offset (get-offset nums)
        repeated-nums-suffix (at-offset-with-reps nums offset 10000)
        first8 (take 8 (run-its repeated-nums-suffix 100))]
    (apply str first8)))

(deftest test-solve
  (is (= "84462026" (solve "03036732577212944063491565474664")))
  (is (= "78725270" (solve "02935109699940807407585447034323")))
  (is (= "53553731" (solve "03081770884921959731165446850517"))))

(defn -main [& _]
  (println (solve (slurp *in*))))
