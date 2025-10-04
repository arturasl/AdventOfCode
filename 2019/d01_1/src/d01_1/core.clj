(ns d01-1.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.java.io :as io]))

(defn find-fuel
  [mass]
  (-> mass (quot 3) (- 2)))

(comment
  (find-fuel 12)
  (find-fuel 14)
  (find-fuel 1969)
  (find-fuel 100756))

(defn solve
  [lines]
  (->> lines
       (map str/trim)
       (filter not-empty)
       (map parse-long)
       (map find-fuel)
       (reduce +)))

(comment
  (solve ["  12" "" "14  "]))

(defn -main
  [& _]
  (with-open [rdr (io/reader "./large.in")]
    (println (solve (line-seq rdr)))))

