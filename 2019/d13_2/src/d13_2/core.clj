(ns d13-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.test :refer [deftest is]]
            [clojure.data.finger-tree :as finger]
            [intcode.core :as code]))

(defn parse-tile-id [id]
  (case id
    0 :empty
    1 :wall
    2 :block
    3 :horizontal-paddle
    4 :ball
    (throw (ex-info (str "Unknown id: " id) {:id id}))))

(defn parse-tile-instruction [[x y id]]
  (if (= [y x] [0 -1])
    [:score id]
    [{:y y :x x} (parse-tile-id id)]))

(defn parse-tile-instructions [output]
  (->> output
       (partition 3)
       (map parse-tile-instruction)))

(deftest test-parse-tile-instructions
  (is (= [[{:y 2 :x 1} :horizontal-paddle] [{:y 5 :x 6} :ball]]
         (parse-tile-instructions [1,2,3,6,5,4]))))

(defn grid->str [grid]
  (let [ys (filter (comp not nil?) (map :y (keys grid)))
        xs (filter (comp not nil?) (map :x (keys grid)))
        [min-y max-y] (map #(apply % ys) [min max])
        [min-x max-x] (map #(apply % xs) [min max])
        height (inc (- max-y min-y))
        width (inc (- max-x min-x))
        drawn (reduce (fn [drawn-so-far [y x]]
                        (assoc-in drawn-so-far [y x]
                                  (case (get grid {:y (+ y min-y) :x (+ x min-x)} :empty)
                                    :empty " "
                                    :wall "█"
                                    :block "■"
                                    :horizontal-paddle "―"
                                    :ball "⬤")))
                      (into [] (repeat height (into [] (repeat width " "))))
                      (for [y (range height) x (range width)] [y x]))]
    (str (str/join "\n" (map str/join drawn)) "\nScore: " (:score grid))))

(defn program->grid [program]
  (->> (:output program)
       (parse-tile-instructions)
       flatten
       (apply hash-map)))

(defn get-item-x [grid item]
  (some #(and (= (second %) item) (:x (first %))) grid))

(defn get-next-move [grid]
  (compare (get-item-x grid :ball) (get-item-x grid :horizontal-paddle)))

(defn solve [s]
  (loop [program (-> s
                     code/str->memory
                     code/init-program
                     (assoc-in [:memory 0] 2)
                     code/exec)
         grid (program->grid program)]
    (println (grid->str grid))
    (if (= (:state program) :halt)
      program
      (do
        (assert (= (:state program) :waiting-read))
        (let [program-wo-output (assoc program :output (finger/double-list))
              next-program (code/exec (code/to->stdin program-wo-output [(get-next-move grid)]))
              next-grid (merge grid (program->grid next-program))]
          (recur next-program next-grid))))))

(defn -main
  [& _]
  (solve (slurp  "./large.in")))
