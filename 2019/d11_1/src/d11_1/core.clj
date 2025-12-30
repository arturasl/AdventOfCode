(ns d11-1.core
  (:gen-class)
  (:require [intcode.core :as code]
            [clojure.data.finger-tree :as finger]))

(defn rotate [dir rotation]
  (assert (get #{0 1} rotation) (str "Got rotation: " rotation))
  (if (= rotation 0)
    {:y (- (:x dir)) :x (:y dir)}
    {:y (:x dir) :x (- (:y dir))}))

(defn move [pos1 pos2]
  (let [coords [:y :x]]
    (zipmap coords (map #(+ (% pos1) (% pos2)) coords))))

(defn solve [s]
  (loop [program
         (-> s
             code/str->memory
             code/init-program)
         grid {}
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

(defn -main
  [& _]
  (println (count (solve (slurp  "./large.in")))))
