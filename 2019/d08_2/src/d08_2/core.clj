(ns d08-2.core
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

(defn list->layers [lst w h]
  (->> (partition (* w h) lst)
       (map (partial partition w))))

(deftest test-list->layers
  (is (= [[[1 2 3] [4 5 6]] [[7 8 9] [0 1 2]]]
         (list->layers [1 2 3 4 5 6 7 8 9 0 1 2] 3 2))))

(defn calc-visible-layer [layers]
  (reduce
   (fn [acc_layer cur_layer]
     (map
      (fn [acc_row cur_row]
        (map
         (fn [acc_cell cur_cell] (if (not= acc_cell 2) acc_cell cur_cell))
         acc_row cur_row))
      acc_layer cur_layer))
   layers))

(deftest test-calc-visible-layer
  (is (= [[0 1] [1 0]]
         (calc-visible-layer  [[[0 2] [2 2]] [[1 1] [2 2]] [[2 2] [1 2]] [[0 0] [0 0]]]))))

(defn layer->str [layer]
  (str/join "\n" (map str/join layer)))

(deftest test-layer->str
  (is (= "01\n10"
         (layer->str  [[0 1] [1 0]]))))

(defn solve [s]
  (-> s
      str->list
      (list->layers width height)
      calc-visible-layer
      layer->str))

(defn -main
  [& _]
  (println (solve (slurp  "./large.in"))))
