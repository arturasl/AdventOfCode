(ns d06-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [clojure.set :as set]
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

(defn find-path-to-root
  ([edges from] (find-path-to-root edges from 0))
  ([edges from dist]
   (let [parents (get edges from)]
     (into {from dist}
           (case (count parents)
             0 {}
             1 (find-path-to-root edges (first parents) (inc dist))
             (throw (ex-info "Too many parents" {:node from})))))))

(deftest test-count->children
  (is (= {"D" 0} (find-path-to-root {"C" #{"B" "E"} "E" #{"D"}} "D")))
  (is (= {"C" 0, "E" 1, "D" 2} (find-path-to-root {"C" #{"E"} "E" #{"D"}} "C"))))

(defn set-keys [dict]
  (set (keys dict)))

(defn solve [lines]
  (let [edges (lines->edges lines)
        path-you (find-path-to-root edges "YOU")
        path-san (find-path-to-root edges "SAN")
        common-nodes (set/intersection (set-keys path-you) (set-keys path-san))]
    (->> common-nodes
         (map #(+ (get path-you %) (get path-san %)))
         (reduce min)
         (+ -2))))

(deftest test-solve
  (is (= 4 (solve ["COM)B" "B)C" "C)D" "D)E" "E)F" "B)G" "G)H" "D)I" "E)J"
                   "J)K" "K)L" "K)YOU" "I)SAN"]))))

(defn -main
  [& _]
  (with-open [rdr (io/reader "./large.in")]
    (println (solve (line-seq rdr)))))
