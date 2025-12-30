(ns d19-1.core
  (:gen-class)
  (:require [intcode.core :as code]))

(defn get-beam-coords [s height width]
  (let [program (->> s
                     code/str->memory
                     code/init-program)]
    (into #{}
          (for [y (range height)
                x (range width)
                :let [in-beam (not
                               (zero?
                                (->> [y x]
                                     (code/to->stdin program)
                                     code/exec
                                     :output
                                     first)))]
                :when in-beam]
            {:y y :x x}))))

(defn solve [s]
  (let [height 50
        width 50
        coords (get-beam-coords s height width)]
    (println (count coords))))

(defn -main
  [& _]
  (solve (slurp *in*)))
