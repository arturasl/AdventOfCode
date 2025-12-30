(ns d19-2.core
  (:gen-class)
  (:require [clojure.data.finger-tree :as finger]
            [intcode.core :as code]))

(defn create-program-in-beam-checker [s]
  (let [program (->> s code/str->memory code/init-program)
        is-in-beam (fn [{:keys [y x]}]
                     (not (zero?
                           (->> [x y]
                                (code/to->stdin program)
                                code/exec
                                :output
                                first))))]
    (memoize is-in-beam)))

(defn calc-ans [{:keys [y x]}]
  (+ (* x 10000) y))

(defn rect-br->tl [config br]
  (let [dim-incl (- (dec (:dim config)))]
    {:y (- (:y br) dim-incl)
     :x (- (:x br) dim-incl)}))

(defn is-rect-tl [config tl]
  (let [dim-incl (dec (:dim config))]
    (every?
     (fn [[dy dx]]
       ((:is-in-beam config) {:y (+ dy (:y tl))
                              :x (+ dx (:x tl))}))
     [[dim-incl 0] [0 dim-incl] [dim-incl dim-incl]])))

(defn get-beam-coords-bfs [config]
  (loop [search-space (finger/double-list (:start-pos config))
         visited #{(:start-pos config)}
         its 0]
    (let [cur-pos (first search-space)
          search-space (rest search-space)
          its (inc its)
          next-poses (for [dy (range -1 2)
                           dx (range -1 2)
                           :let [pos {:y (+ dy (:y cur-pos))
                                      :x (+ dx (:x cur-pos))}]
                           :when (and (<= 0 (:y pos))
                                      (<= 0 (:x pos))
                                      (not (contains? visited pos)))
                           :when ((:is-in-beam config) pos)]
                       pos)
          rects (->> next-poses
                     (map (partial rect-br->tl config))
                     (filter (partial is-rect-tl config)))]
      (when (zero? (mod its 100))
        (println "its:" its
                 "visited:" (count visited)
                 "cur-pos:" cur-pos
                 "search-space:" (count search-space)))
      (if (not-empty rects) rects
          (recur (into search-space next-poses)
                 (into visited next-poses)
                 its)))))

(defn solve [s]
  (let [config {:is-in-beam (create-program-in-beam-checker s)
                :dim 100
                :start-pos {:y 10, :x 10}}
        result (map calc-ans (get-beam-coords-bfs config))
        _ (assert (= 1 (count result)))]
    (first result)))

(defn -main
  [& _]
  (println (solve (slurp *in*))))
