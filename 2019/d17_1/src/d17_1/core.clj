(ns d17-1.core
  (:gen-class)
  (:require [clojure.string :as str]
            [intcode.core :as code]))

(defn solve [s]
  (let [grid
        (->> s
             code/str->memory
             code/init-program
             code/exec
             :output
             (map char)
             (apply str)
             (str/split-lines))
        height (count grid)
        width (count (get grid 0))
        intersections (for [y (range height)
                            x (range width)
                            :when (every? (fn [[y x]] (= (get (get grid y) x) \#))
                                          [[y x] [(inc y) x] [(dec y) x] [y (inc x)] [y (dec x)]])]
                        {:y y :x x})]
    (reduce + (map (fn [{:keys [y x]}] (* y x)) intersections))))

(defn -main
  [& _]
  (println (solve (slurp  "./large.in"))))
