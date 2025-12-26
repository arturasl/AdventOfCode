(ns d24-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]))

(def ^:const coords-around
  [{:y -1 :x 0} {:y 1 :x 0}
   {:y 0 :x -1} {:y 0 :x 1}])

(defn add-coords [lhs rhs]
  (merge-with + lhs rhs))

(deftest test-add-coords
  (is (= {:y 14 :x 3} (add-coords {:y 5 :x 2} {:y 9 :x 1}))))

(defn str->state [s]
  (let [lines (->> s
                   str/split-lines
                   (map str/trim)
                   (remove empty?)
                   vec)
        height (count lines)
        _ (assert (not (zero? height)))
        width (count lines)
        _ (assert (every? #(= (count %) width) lines))
        bugs (into #{}
                   (for [y (range height)
                         x (range width)
                         :let [cell (get-in lines [y x])]
                         :when (= cell \#)]
                     {:y y :x x :level 0}))]
    {:height height
     :width width
     :bugs bugs}))

(def create-level-jumps
  (memoize
   (fn [height width]
     (let [mid [(quot height 2) (quot width 2)]
           small-big-pairs (concat
                            (for [y (range height)]
                              [{:y y :x 0} {:y (get mid 0) :x (dec (get mid 1))}])
                            (for [y (range height)]
                              [{:y y :x (dec width)} {:y (get mid 0) :x (inc (get mid 1))}])
                            (for [x (range width)]
                              [{:y 0 :x x} {:y (dec (get mid 0)) :x (get mid 1)}])
                            (for [x (range width)]
                              [{:y (dec height) :x x} {:y (inc (get mid 0)) :x (get mid 1)}]))
           bidi-pairs (concat
                       (map (fn [[k v]]
                              [k (assoc v :level -1)])
                            small-big-pairs)
                       (map (fn [[k v]]
                              [v (assoc k :level 1)])
                            small-big-pairs))
           bidi-map (->> bidi-pairs
                         (group-by first)
                         (map
                          (fn [[k vals]]
                            [k (into #{} (map second vals))]))
                         (into {}))]
       bidi-map))))

(defn get-coords-around [{:keys [height width]} coord]
  (let [mid [(quot height 2) (quot width 2)]
        direct (remove (fn [{:keys [y x]}]
                         (or (< y 0)
                             (< x 0)
                             (>= y height)
                             (>= x width)
                             (= [y x] mid)))
                       (map (fn [dc] (add-coords dc coord)) coords-around))
        bidi-map (create-level-jumps height width)
        additional (map (fn [add-coord]
                          (update add-coord :level (partial + (:level coord))))
                        (get bidi-map (select-keys coord [:y :x]) []))]
    (into #{} (concat direct additional))))

(deftest test-get-coords-around
  (is (= #{{:y 0 :x 1 :level 0} {:y 1 :x 0 :level 0} {:y 1 :x 2 :level 0} {:y 2 :x 1 :level 0}}
         (get-coords-around {:height 5 :width 5} {:y 1 :x 1 :level 0})))
  (is (= #{{:y 0 :x 0 :level 0} {:y 2 :x 0 :level 0} {:y 1 :x 1 :level 0}
           {:y 2 :x 1 :level -1}}
         (get-coords-around {:height 5 :width 5} {:y 1 :x 0 :level 0})))
  (is (= #{{:y 0 :x 4 :level 0} {:y 2 :x 4 :level 0} {:y 1 :x 3 :level 0}
           {:y 2 :x 3 :level -1}}
         (get-coords-around {:height 5 :width 5} {:y 1 :x 4 :level 0})))
  (is (= #{{:y 1 :x 0 :level 0} {:y 0 :x 1 :level 0}
           {:y 1 :x 2 :level -1} {:y 2 :x 1 :level -1}}
         (get-coords-around {:height 5 :width 5} {:y 0 :x 0 :level 0})))
  (is (= #{{:y 4 :x 1 :level 0} {:y 3 :x 2 :level 0} {:y 4 :x 3 :level 0}
           {:y 3 :x 2 :level -1}}
         (get-coords-around {:height 5 :width 5} {:y 4 :x 2 :level 0})))
  (is (= #{{:y 2 :x 0 :level 0} {:y 1 :x 1 :level 0} {:y 3 :x 1 :level 0}
           {:y 0 :x 0 :level 1} {:y 1 :x 0 :level 1} {:y 2 :x 0 :level 1}
           {:y 3 :x 0 :level 1} {:y 4 :x 0 :level 1}}
         (get-coords-around {:height 5 :width 5} {:y 2 :x 1 :level 0}))))

(defn calc-next-state [s]
  (let [bugs (:bugs s)
        interesting-coords (into #{}
                                 (concat
                                  bugs
                                  (apply concat (map #(get-coords-around s %) bugs))))
        next-bugs (into #{}
                        (for [coord interesting-coords
                              :let [is-bug (contains? bugs coord)
                                    num-bugs-around (->> coord
                                                         (get-coords-around s)
                                                         (map #(contains? bugs %))
                                                         (filter identity)
                                                         count)]
                              :when (or (and is-bug
                                             (= 1 num-bugs-around))
                                        (and (not is-bug)
                                             (contains? #{1 2} num-bugs-around)))]
                          coord))]
    (assoc s :bugs next-bugs)))

(defn solve [s]
  (let [state (str->state s)
        final-state (reduce
                     (fn [state _] (calc-next-state state))
                     state
                     (range 200))]
    (println (count (:bugs final-state)))))

(defn -main
  [& _]
  (solve (slurp *in*)))
