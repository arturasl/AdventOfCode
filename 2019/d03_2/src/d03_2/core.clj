(ns d03-2.core
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

(defn walk [instructions]
  (reduce
   (fn [path, inst] (into path (move (last path) inst)))
   [[0 0]]
   instructions))

(comment
  (walk (str->instructions "R8,U5,L5,D3")))

(defn assoc-if-new [m k v]
  (if (contains? m k)
    m
    (assoc m k v)))

(comment
  (assoc-if-new {:a 1} :a 2)
  (assoc-if-new {:a 1} :b 2))

(defn find-shortest [instructions]
  (let [path (walk instructions)]
    (reduce
     (fn [shortest, [pos dist]] (assoc-if-new shortest pos dist))
     {}
     (map vector path (range)))))

(comment
  (assoc-in {:b 2} [:a] 3))

(comment
  (find-shortest (str->instructions "R8,U5,L5,D3")))

(defn solve [lines]
  (let [[lhs rhs]
        (->> lines
             (map str/trim)
             (remove empty?)
             (map str->instructions)
             (map find-shortest))
        ks (set/intersection (set (keys lhs)) (set (keys rhs)))]
    (->> (disj ks [0 0])
         (map #(+ (get lhs %) (get rhs %)))
         (reduce min))))

(comment
  (solve ["R8,U5,L5,D3" "U7,R6,D4,L4"])
  (solve ["R75,D30,R83,U83,L12,D49,R71,U7,L72" "U62,R66,U55,R34,D71,R55,D58,R83"]))

(defn -main
  [& _]
  (with-open [rdr (io/reader "./large.in")]
    (println (solve (line-seq rdr)))))
