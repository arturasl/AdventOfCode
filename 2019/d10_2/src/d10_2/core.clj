(ns d10-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [clojure.java.io :as io]
            [clojure.math.numeric-tower :as math]
            [clojure.data.finger-tree :as ft]))

(defn get-asteroid-positions [s]
  (->> s
       (map str/trim)
       (remove empty?)
       (map (fn [y row]
              (map (fn [x cell] {:y y :x x :cell cell}) (range) row))
            (range))
       flatten
       (filter #(= (:cell %) \#))
       (map #(dissoc % :cell))))

(deftest test-get-asteroid-positions
  (is (= [{:y 0 :x 0} {:y 0 :x 3} {:y 0 :x 7} {:y 1 :x 0}]
         (get-asteroid-positions ["#..#...#" "# "]))))

(defn angle-between [{lhs_y :y lhs_x :x} {rhs_y :y rhs_x :x}]
  (let [diff-y (- lhs_y rhs_y)
        diff-x (- rhs_x lhs_x)
        gcd (math/gcd (abs diff-y) (abs diff-x))]
    (assert (not (and (= diff-y 0) (= diff-x 0))))
    (cond
      (= diff-y 0) [0 (compare diff-x 0)]
      (= diff-x 0) [(compare diff-y 0) 0]
      :else [(/ diff-y gcd) (/ diff-x gcd)])))

(deftest test-angle-between
  (is (= [-1 2] (angle-between {:y 7 :x 10} {:y 12 :x 20})))
  (is (= [0 1] (angle-between {:y 12 :x 10} {:y 12 :x 20})))
  (is (= [-1 0] (angle-between {:y 7 :x 10} {:y 12 :x 10}))))

(defn sq [num] (* num num))

(defn calc-sq-dist [{lhs_y :y lhs_x :x} {rhs_y :y rhs_x :x}]
  (+ (sq (- rhs_y lhs_y)) (sq (- rhs_x lhs_x))))

(defn calc-angles-and-distances [positions]
  (let [pos-to-other (for [lhs positions]
                       {:pos lhs
                        :other
                        (for [rhs positions :when (not= lhs rhs)]
                          {:pos rhs
                           :sq-dist (calc-sq-dist lhs rhs)
                           :angle (angle-between lhs rhs)})})
        with-uniq-angles (map (fn [m] (assoc m
                                             :num-uniq-angles
                                             (->> (:other m)
                                                  (map :angle)
                                                  (into #{}) count)))
                              pos-to-other)]
    with-uniq-angles))

(defn angl-to-quad [[t b]]
  (cond
    (= b 0) (if (= t 1) 1 5)
    (= t 0) (if (= b 1) 3 7)
    (and (> t 0) (> b 0)) 2
    (and (< t 0) (> b 0)) 4
    (and (< t 0) (< b 0)) 6
    :else 8))

(deftest test-angl-to-quad
  (is (= 1 (angl-to-quad [1 0])))
  (is (= 5 (angl-to-quad [-1 0])))
  (is (= 3 (angl-to-quad [0 1])))
  (is (= 7 (angl-to-quad [0 -1])))
  (is (= 2 (angl-to-quad [2 1])))
  (is (= 4 (angl-to-quad [-2 1])))
  (is (= 6 (angl-to-quad [-2 -1])))
  (is (= 8 (angl-to-quad [2 -1]))))

(defn compare-angles [lhs rhs]
  (let [lhs-quad (angl-to-quad lhs)
        rhs-quad (angl-to-quad rhs)
        cmp-quads (compare lhs-quad rhs-quad)
        safe-div #(cond
                    (zero? (first %)) (- (abs (second %)))
                    (zero? (second %)) (- (abs (first %)))
                    :else (/ (first %) (second %)))]
    (if (not= cmp-quads 0)
      cmp-quads
      (- (compare (safe-div lhs) (safe-div rhs))))))

(deftest test-compare-angles
  (is (= -1 (compare-angles [2 1] [1 1])))
  (is (= -1 (compare-angles [1 0] [2 0])))
  (is (= -1 (compare-angles [0 -1] [0 -2])))
  (is (= -1 (compare-angles [1 0] [-1 0])))
  (is (= -1 (compare-angles [1 0] [0 0])))
  (is (= -1 (compare-angles [1 0] [2 -1]))))

(defn flatten-angle-groupings [others-by-angle]
  (let  [sorted-angle (->> others-by-angle
                           (into [])
                           (sort #(compare-angles (first %1) (first %2))))
         sorted-by-dist (map #(update % 1 (partial sort-by :sq-dist)) sorted-angle)
         dropped-angles (into (ft/double-list) (map second sorted-by-dist))]
    (loop [left dropped-angles
           collected []]
      (if (empty? left)
        collected
        (let [first-stack (first left)
              rest-of-stack (rest first-stack)
              rest-left (rest left)
              next-left (if (empty? rest-of-stack) rest-left (conj rest-of-stack rest-left))
              next-collected (conj collected (first first-stack))]
          (recur next-left next-collected))))))

(defn solve [s]
  (let [positions (get-asteroid-positions s)
        angles-and-distances (calc-angles-and-distances positions)
        best (apply (partial max-key :num-uniq-angles) angles-and-distances)
        others-by-angle (group-by :angle (:other best))
        flattened (flatten-angle-groupings others-by-angle)
        two-hundreths (:pos (nth flattened 199))]
    (+ (* (:x two-hundreths) 100) (:y two-hundreths))))

(deftest test-solve
  (is (= 8
         (solve [".#..##.###...#######"
                 "##.############..##."
                 ".#.######.########.#"
                 ".###.#######.####.#."
                 "#####.##.#.##.###.##"
                 "..#####..#.#########"
                 "####################"
                 "#.####....###.#.#.##"
                 "##.#################"
                 "#####.##.###..####.."
                 "..######..##.#######"
                 "####.##.####...##..#"
                 ".#####..#.######.###"
                 "##...#.##########..."
                 "#.##########.#######"
                 ".####.#.###.###.#.##"
                 "....##.##.###..#####"
                 ".#.#.###########.###"
                 "#.#.#.#####.####.###"
                 "###.##.####.##.#..##"]))))

(defn -main
  [& _]
  (with-open [rdr (io/reader "./large.in")]
    (println (solve (line-seq rdr)))))
