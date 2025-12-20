(ns d18-1.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [clojure.data.finger-tree :as finger]))

(defn str->grid [s]
  (let [grid (->> s
                  (str/split-lines)
                  (map str/trim)
                  (remove empty?)
                  (map
                   (fn [line]
                     (map
                      (fn [char]
                        (cond
                          (= char \#) {:type :wall}
                          (= char \.) {:type :empty}
                          (= char \@) {:type :self}
                          (Character/isUpperCase char)
                          {:type :door :val char}
                          (Character/isLowerCase char)
                          {:type :key :val (Character/toUpperCase char)}
                          :else (throw (ex-info "Unknown symbol" {:char char}))))
                      line)))
                  (map vec)
                  vec)
        height (count grid)
        width (count (get grid 0))
        _ (assert (every? #(= (count %) width) grid) "Inconsistent width")
        keys (into #{}
                   (for [y (range height)
                         x (range width)
                         :let [cell (get-in grid [y x])]
                         :when (= (:type cell) :key)]
                     (:val cell)))
        starts (for [y (range height)
                     x (range width)
                     :let [cell (get-in grid [y x])]
                     :when (= (:type cell) :self)]
                 [y x])
        _ (assert (= (count starts) 1) "There should be only one start")
        start (first starts)]
    {:height height
     :width width
     :keys keys
     :start start
     :grid (assoc-in grid [(first start) (second start)] {:type :empty})}))

(deftest test-str->grid
  (is (=
       {:height 1
        :width 5
        :keys #{\A}
        :start [0 1]
        :grid [[{:type :empty}
                {:type :empty}
                {:type :wall}
                {:type :key :val \A}
                {:type :door :val \A}]]}
       (str->grid ".@#aA"))))

(defn solve [s]
  (let [grid (str->grid s)]
    (loop [states (finger/double-list {:pos (:start grid) :keys #{} :dist 0})
           visited #{{:pos (:start grid) :keys #{}}}]
      (let [cur-state (first states)
            _ (assert (not (nil? cur-state)) "Did an exhaustive search and no solution was found")
            [cur-y cur-x] (:pos cur-state)
            cur-keys (:keys cur-state)
            states (rest states)
            next-states (for [y (range -1 2)
                              x (range -1 2)
                              :let [cell-pos [(+ y cur-y) (+ x cur-x)]
                                    cell (get-in (:grid grid) cell-pos)
                                    cell-type (:type cell)
                                    cell-keys (if (= cell-type :key)
                                                (conj cur-keys (:val cell))
                                                cur-keys)]
                              :when (and
                                     (not (nil? cell))
                                     (not (zero? (bit-xor
                                                  (if (zero? y) 1 0)
                                                  (if (zero? x) 1 0))))
                                     (not= cell-type :wall)
                                     (or (not= cell-type :door)
                                         (contains? cell-keys (:val cell)))
                                     (not (contains? visited {:pos cell-pos :keys cell-keys})))]
                          {:pos cell-pos
                           :keys cell-keys
                           :dist (inc (:dist cur-state))})]
        (if (= cur-keys (:keys grid))
          (:dist cur-state)
          (recur
           (into states next-states)
           (into visited (map (fn [{:keys [pos keys]}] {:pos pos :keys keys}) next-states))))))))

(deftest test-solve
  (is (= 8
         (solve "#########\n#b.A.@.a#\n#########")))
  (is (= 86
         (solve (str/join \newline
                          ["########################"
                           "#f.D.E.e.C.b.A.@.a.B.c.#"
                           "######################.#"
                           "#d.....................#"
                           "########################"]))))
  (is (= 132
         (solve (str/join \newline
                          ["########################"
                           "#...............b.C.D.f#"
                           "#.######################"
                           "#.....@.a.B.c.d.A.e.F.g#"
                           "########################"])))))

(defn -main
  [& _]
  (println (solve (slurp  "./large.in"))))
