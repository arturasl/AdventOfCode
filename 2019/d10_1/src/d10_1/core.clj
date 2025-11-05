(ns d10-1.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [clojure.java.io :as io]
            [clojure.math.numeric-tower :as math]))

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
        diff-x (- lhs_x rhs_x)
        gcd (math/gcd (abs diff-y) (abs diff-x))]
    (assert (not (and (= diff-y 0) (= diff-x 0))))
    (cond
      (= diff-y 0) [0 (compare diff-x 0)]
      (= diff-x 0) [(compare diff-y 0) 0]
      :else [(/ diff-y gcd) (/ diff-x gcd)])))

(deftest test-angle-between
  (is (= [-1 -2] (angle-between {:y 7 :x 10} {:y 12 :x 20})))
  (is (= [0 -1] (angle-between {:y 12 :x 10} {:y 12 :x 20})))
  (is (= [-1 0] (angle-between {:y 7 :x 10} {:y 12 :x 10}))))

(defn solve [s]
  (let [positions (get-asteroid-positions s)
        pos-to-angles (for [lhs positions]
                        [lhs
                         (for [rhs positions :when (not= lhs rhs)]
                           (angle-between lhs rhs))])
        pos-to-num-angles (map (fn [[pos angles]] [pos (count (into #{} angles))])
                               pos-to-angles)
        max-angle (reduce max (map second pos-to-num-angles))]
    max-angle))

(deftest test-solve
  (is (= 8
         (solve [".#..#"
                 "....."
                 "#####"
                 "....#"
                 "...##"])))

  (is (= 33
         (solve ["......#.#."
                 "#..#.#...."
                 "..#######."
                 ".#.#.###.."
                 ".#..#....."
                 "..#....#.#"
                 "#..#....#."
                 ".##.#..###"
                 "##...#..#."
                 ".#....####"])))

  (is (= 35
         (solve ["#.#...#.#."
                 ".###....#."
                 ".#....#..."
                 "##.#.#.#.#"
                 "....#.#.#."
                 ".##..###.#"
                 "..#...##.."
                 "..##....##"
                 "......#..."
                 ".####.###."])))

  (is (= 41
         (solve [".#..#..###"
                 "####.###.#"
                 "....###.#."
                 "..###.##.#"
                 "##.##.#.#."
                 "....###..#"
                 "..#.#..#.#"
                 "#..#.#.###"
                 ".##...##.#"
                 ".....#.#.."]))))

(defn -main
  [& _]
  (with-open [rdr (io/reader "./large.in")]
    (println (solve (line-seq rdr)))))
