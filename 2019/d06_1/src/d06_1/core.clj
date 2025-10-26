(ns d06-1.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [clojure.java.io :as io]))

(defn lines->edges [lines]
  (->> lines
       (map #(str/replace % #"\s" ""))
       (remove empty?)
       (map #(str/split % #"\)"))
       (map (fn [[from to]] [to from]))
       (group-by first)
       (map (fn [[k v]] [k (into #{} (map second v))]))
       (into {})))

(deftest test-lines->edges
  (is (= {"C" #{"B" "E"} "E" #{"D"}}
         (lines->edges ["B)C" "E)C" "D  ) E" " "]))))

(def count->children
  (memoize
   (fn [edges cur]
     (if (not (contains? edges cur)) 0
         (reduce #(+ % 1 (count->children edges %2)) 0 (get edges cur))))))

(deftest test-count->children
  (is (= 0 (count->children {"C" #{"B" "E"} "E" #{"D"}} "D")))
  (is (= 1 (count->children {"C" #{"B" "E"} "E" #{"D"}} "E")))
  (is (= 3 (count->children {"C" #{"B" "E"} "E" #{"D"}} "C"))))

(defn solve [lines]
  (let [edges (lines->edges lines)
        non-leaf (keys edges)]
    (->> non-leaf
         (map (partial count->children edges))
         (reduce +))))

(deftest test-solve
  (is (= 42 (solve ["COM)B" "B)C" "C)D" "D)E" "E)F" "B)G" "G)H" "D)I" "E)J"
                    "J)K" "K)L"]))))

(defn -main
  [& _]
  (with-open [rdr (io/reader "./large.in")]
    (println (solve (line-seq rdr)))))
