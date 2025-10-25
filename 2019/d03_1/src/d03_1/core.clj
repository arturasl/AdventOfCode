(ns d03-1.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.set :as set]
            [clojure.java.io :as io]))

(defn ch->dir [s]
  (case s
    \R [0 1]
    \L [0 -1]
    \U [-1 0]
    \D [1 0]
    (throw (RuntimeException. (str "Unknown dir: '" s "'")))))

(defn str->instructions [s]
  (->> (str/split s #",")
       (map str/trim)
       (remove empty?)
       (map (fn [val]
              {:dir (ch->dir (get val 0))
               :steps (parse-long (subs val 1))}))
       vec))

(comment
  (str->instructions "R8,U5,L5,D3"))

(defn move [pos {:keys [dir steps]}]
  (for [i (range 1 (+ 1 steps))]
    (map (fn [p d] (+ p (* d i)))
         pos dir)))

(comment
  (move [1 2] {:dir [-1 0] :steps 3})
  (move [1 2] {:dir [0 -1] :steps 3}))

(defn calc-path [instructions]
  (rest
   (reduce
    (fn [path, inst] (into path (move (last path) inst)))
    [[0 0]]
    instructions)))

(comment
  (calc-path (str->instructions "R8,U5,L5,D3")))

(defn find-intersections [lines]
  (->> lines
       (map str/trim)
       (remove empty?)
       (map str->instructions)
       (map calc-path)
       (map #(into #{} %))
       (apply set/intersection)))

(comment
  (find-intersections ["R8,U5,L5,D3" "U7,R6,D4,L4"]))

(defn calc-manhattan-dist [pos]
  (reduce + (map abs pos)))

(comment
  (calc-manhattan-dist [1 2])
  (calc-manhattan-dist [-1 -2]));

(defn solve [lines]
  (->> lines
       find-intersections
       (map calc-manhattan-dist)
       (apply min)))

(comment
  (solve ["R8,U5,L5,D3" "U7,R6,D4,L4"]))

(defn -main
  [& _]
  (with-open [rdr (io/reader "./large.in")]
    (println (solve (line-seq rdr)))))
