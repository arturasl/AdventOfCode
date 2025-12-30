(ns d11-2.core
  (:gen-class)
  (:require [clojure.string :as str]
            [clojure.data.finger-tree :as finger]
            [intcode.core :as code]))

(defn rotate [dir rotation]
  (assert (get #{0 1} rotation) (str "Got rotation: " rotation))
  (if (= rotation 0)
    {:y (- (:x dir)) :x (:y dir)}
    {:y (:x dir) :x (- (:y dir))}))

(defn move [pos1 pos2]
  (let [coords [:y :x]]
    (zipmap coords (map #(+ (% pos1) (% pos2)) coords))))

(defn walk [program]
  (loop [program program
         grid {{:y 0 :x 0} 1}
         cur-pos {:y 0 :x 0}
         dir {:y -1 :x 0}]
    (let [next-program (-> program
                           (code/to->stdin [(get grid cur-pos 0)])
                           code/exec)]
      (if (= (:state next-program) :halt)
        grid
        (let [[color rotation] (get next-program :output)
              next-program (assoc next-program :output (finger/double-list))
              next-grid (assoc grid cur-pos color)
              next-dir (rotate dir rotation)
              next-pos (move cur-pos next-dir)]
          (recur next-program next-grid next-pos next-dir))))))

(defn draw [grid]
  (let [ys (map :y (keys grid))
        xs (map :x (keys grid))
        [min-y max-y] (map #(apply % ys) [min max])
        [min-x max-x] (map #(apply % xs) [min max])
        height (inc (- max-y min-y))
        width (inc (- max-x min-x))
        drawn (reduce (fn [drawn-so-far [y x]]
                        (assoc-in drawn-so-far [y x] "#"))
                      (into [] (repeat height (into [] (repeat width " "))))
                      (for [y (range height) x (range width)
                            :when (= (get grid {:y (+ y min-y) :x (+ x min-x)} 0) 1)]
                        [y x]))]
    (str/join "\n" (map str/join drawn))))

(defn solve [s]
  (draw (walk (-> s code/str->memory code/init-program))))

(defn -main
  [& _]
  (println (solve (slurp  "./large.in"))))
