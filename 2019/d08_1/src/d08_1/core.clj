(ns d08-1.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]))

(def ^:const width 25)
(def ^:const height 6)

(defn str->list [s]
  (->> (map str s)
       (map str/trim)
       (remove empty?)
       (map parse-long)
       (into [])))

(deftest test-str->list
  (is (= [1 2 3] (str->list "12\n3 "))))

(defn find-layer-frequencies [lst w h]
  (->> (partition (* w h) lst)
       (map frequencies)))

(deftest test-find-layer-frequencies
  (is (= [{1 2, 3 1, 4 1, 5 1, 6 1} {7 1, 8 1, 9 1, 0 1, 1 1, 2 1}]
         (find-layer-frequencies (str->list "113456789012") 3 2))))

(defn find-best-layer [lst w h]
  (->>  (find-layer-frequencies lst w h)
        (apply min-key #(get % 0 0))
        (into {})))

(deftest test-find-best-layer
  (is (= [{1 2, 3 1, 4 1, 5 1, 6 1}]
         (find-best-layer (str->list "113456789012") 3 2))))

(defn solve [s]
  (let [layer (find-best-layer (str->list s) width height)]
    (* (get layer 1) (get layer 2))))

(defn -main
  [& _]
  (println (solve (slurp  "./large.in"))))
