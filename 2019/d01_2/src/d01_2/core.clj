(ns d01-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.java.io :as io]))

(def find-fuel
  (memoize
   (fn [mass]
     (let [fuel (-> mass (quot 3) (- 2))]
       (if (<= fuel 0)
         0
         (+ fuel (find-fuel fuel)))))))

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

