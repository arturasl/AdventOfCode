(ns d13-1.core
  (:gen-class)
  (:require [clojure.test :refer [deftest is]]
            [intcode.core :as code]))

(defn parse-tile-id [id]
  (case id
    0 :empty
    1 :wall
    2 :block
    3 :horizontal-paddle
    4 :ball
    (throw (ex-info "Unknown id: " {:id id}))))

(defn parse-tile-instruction [[x y id]]
  [{:y y :x x} (parse-tile-id id)])

(defn parse-tile-instructions [output]
  (->> output
       (partition 3)
       (map parse-tile-instruction)))

(deftest test-parse-tile-instructions
  (is (= [[{:y 2 :x 1} :horizontal-paddle] [{:y 5 :x 6} :ball]]
         (parse-tile-instructions [1,2,3,6,5,4]))))

(defn solve [s]
  (let [output
        (-> s
            code/str->memory
            code/init-program
            code/exec
            :output)
        grid (->> output
                  (parse-tile-instructions)
                  flatten
                  (apply hash-map))]
    (count (filter #(= % :block) (vals grid)))))

(defn -main
  [& _]
  (println (solve (slurp  "./large.in"))))
