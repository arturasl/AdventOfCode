(ns d20-1.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.data.priority-map :refer [priority-map]]))

(defn find-portals
  ([grid width height]
   (concat (find-portals grid width height :horizontal)
           (find-portals grid width height :vertical)))
  ([grid width height tp]
   (for [y (range (- height (if (= tp :vertical) 2 0)))
         x (range (- width (if (= tp :horizontal) 2 0)))
         :let [slice (apply
                      str
                      (for [dc (range 3)]
                        (get-in grid [(+ y (if (= tp :vertical) dc 0))
                                      (+ x (if (= tp :horizontal) dc 0))])))]
         :when (re-matches #"\.\w\w|\w\w\." slice)
         :let [portal (apply str (filter #(not= % \.) slice))
               p-y (if (or (= tp :horizontal) (= (first slice) \.)) y (+ y 2))
               p-x (if (or (= tp :vertical) (= (first slice) \.)) x (+ x 2))]]
     {:portal portal :y p-y :x p-x})))

(defn compute-portal-to-poses [portals]
  (let [result
        (->> portals
             (group-by :portal)
             (map (fn [[k poses]]
                    [k (vec (map #(select-keys % [:y :x]) poses))]))
             (into {}))
        _ (assert (every?
                   (fn [[portal poses]]
                     (or (= (count poses) 2)
                         (and (= (count poses) 1)
                              (contains? #{"AA" "ZZ"} portal))))
                   result))]
    result))

(defn compute-pos-to-portal [portals]
  (let [result
        (->> portals
             (group-by #(select-keys % [:y :x]))
             (map (fn [[pos portals]]
                    (assert (= (count portals) 1))
                    [pos (:portal (first portals))]))
             (into {}))]
    result))

(defn str->grid [s]
  (->> s
       (str/split-lines)
       (remove empty?)
       (map vec)
       vec))

(defn pos-maybe-via-portal [pos portal-to-poses pos-to-portal]
  (let [portal (get pos-to-portal pos)
        other-pos (remove #(= % pos) (get portal-to-poses portal))]
    (if (not-empty other-pos) {:pos (first other-pos) :cost 2}
        {:pos pos :cost 1})))

(def ^:const max-dist-ever 1000000000)

(defn find-next-states [grid cur-pos cur-dist visited portal-to-poses pos-to-portal]
  (let [next-states (for [dy (range -1 2)
                          dx (range -1 2)
                          :when (not (zero? (bit-xor
                                             (if (zero? dy) 1 0)
                                             (if (zero? dx) 1 0))))
                          :let [next-pos {:y (+ dy (:y cur-pos))
                                          :x (+ dx (:x cur-pos))}
                                cell (get-in grid [(:y next-pos) (:x next-pos)])]
                          :when (= cell \.)
                          :let [jump (pos-maybe-via-portal next-pos portal-to-poses pos-to-portal)
                                next-pos (:pos jump)
                                next-dist (+ (:cost jump) cur-dist)]
                          :when (< next-dist (get visited next-pos max-dist-ever))]
                      [next-pos next-dist])
        best-states (->> next-states
                         (group-by first)
                         (map (fn [[pos grouped]] [pos (map second grouped)]))
                         (map (fn [[pos dists]] [pos (reduce min dists)])))]
    best-states))

(defn solve [s]
  (let [grid (str->grid s)
        height (count grid)
        width (count (first grid))
        portals (find-portals grid width height)
        portal-to-poses (compute-portal-to-poses portals)
        pos-to-portal (compute-pos-to-portal portals)
        start (first (get portal-to-poses "AA"))
        finish (first (get portal-to-poses "ZZ"))]
    (loop [search-space (priority-map start 0)
           global {:visited {start 0}}]
      (assert (not-empty search-space))
      (let [[cur-pos cur-dist] (peek search-space)
            search-space (pop search-space)]
        (if (= cur-pos finish) cur-dist
            (let [next-states (find-next-states
                               grid
                               cur-pos
                               cur-dist
                               (:visited global)
                               portal-to-poses
                               pos-to-portal)
                  next-global (update global :visited
                                      (fn [visited]
                                        (into visited next-states)))
                  next-search-space (into search-space next-states)]
              (recur next-search-space next-global)))))))

(defn -main
  [& _]
  (println (solve (slurp *in*))))
